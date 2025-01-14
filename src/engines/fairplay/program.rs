use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Error};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use libc::{self, c_void, mmap, munmap, open, close, fstat, MAP_FAILED, O_RDONLY, O_RDWR, MAP_PRIVATE, MAP_SHARED, PROT_READ, PROT_WRITE, PROT_EXEC};
use mach::vm_types::vm_address_t;
use mach::message::mach_msg_type_number_t;
use mach::kern_return::kern_return_t;
use mach::mach_port::mach_port_t;
use mach::task::task_t;

#[repr(C)]
struct EncryptionInfoCommand64 {
    cmd: u32,
    cmdsize: u32,
    cryptoff: u32,
    cryptsize: u32,
    cryptid: u32,
    pad: u32,
}

extern "C" {
    fn mremap_encrypted(base: *mut c_void, size: usize, cryptid: u32, cputype: u32, cpusubtype: u32) -> libc::c_int;
}

fn copy(src: &str, dest: &str) -> io::Result<()> {
    if src == dest {
        return Err(io::Error::new(io::ErrorKind::Other, "Source and destination are the same"));
    }

    let mut src_fp = File::open(src)?;
    let mut dest_fp = File::create(dest)?;

    let mut buffer = [0u8; 1];
    loop {
        match src_fp.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => dest_fp.write_all(&buffer)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn unprotect(fd: RawFd, dupe: &mut [u8], info: &EncryptionInfoCommand64) -> io::Result<()> {
    unsafe {
        let base = mmap(ptr::null_mut(), info.cryptsize as usize, PROT_READ | PROT_EXEC, MAP_PRIVATE, fd, info.cryptoff as libc::off_t);
        if base == MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

        let error = mremap_encrypted(base, info.cryptsize as usize, info.cryptid, mach::cpu_type::CPU_TYPE_ARM64, mach::cpu_type::CPU_SUBTYPE_ARM64_ALL);
        if error != 0 {
            munmap(base, info.cryptsize as usize);
            return Err(io::Error::last_os_error());
        }

        ptr::copy_nonoverlapping(base as *const u8, dupe.as_mut_ptr().add(info.cryptoff as usize), info.cryptsize as usize);
        munmap(base, info.cryptsize as usize);
    }

    Ok(())
}

fn map(path: &str, mutable: bool) -> io::Result<(Vec<u8>, usize)> {
    let flags = if mutable { O_RDWR } else { O_RDONLY };
    let fd = unsafe { open(CString::new(path)?.as_ptr(), flags) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    let mut stat: libc::stat = unsafe { mem::zeroed() };
    if unsafe { fstat(fd, &mut stat) } < 0 {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    let size = stat.st_size as usize;
    let prot = if mutable { PROT_READ | PROT_WRITE } else { PROT_READ };
    let map_flags = if mutable { MAP_SHARED } else { MAP_PRIVATE };
    let base = unsafe { mmap(ptr::null_mut(), size, prot, map_flags, fd, 0) };
    if base == MAP_FAILED {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    unsafe { close(fd) };

    let data = unsafe { slice::from_raw_parts(base as *const u8, size) }.to_vec();
    Ok((data, size))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} src dest", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not enough arguments"));
    }

    let (base, base_size) = map(&args[1], false)?;
    copy(&args[1], &args[2])?;

    let (mut dupe, dupe_size) = map(&args[2], true)?;
    if base_size != dupe_size {
        return Err(io::Error::new(io::ErrorKind::Other, "File sizes do not match"));
    }

    let header = unsafe { &*(base.as_ptr() as *const mach::mach_header_64) };
    assert!(header.magic == mach::MH_MAGIC_64);
    assert!(header.cputype == mach::cpu_type::CPU_TYPE_ARM64);
    assert!(header.cpusubtype == mach::cpu_type::CPU_SUBTYPE_ARM64_ALL);

    let mut offset = mem::size_of::<mach::mach_header_64>();
    for _ in 0..header.ncmds {
        let command = unsafe { &*(base[offset..].as_ptr() as *const mach::load_command) };
        if command.cmd == mach::LC_ENCRYPTION_INFO_64 {
            let encryption_info = unsafe { &*(base[offset..].as_ptr() as *const EncryptionInfoCommand64) };
            unprotect(fd, &mut dupe, encryption_info)?;
            let encryption_info_mut = unsafe { &mut *(dupe[offset..].as_mut_ptr() as *mut EncryptionInfoCommand64) };
            encryption_info_mut.cryptid = 0;
            break;
        }
        offset += command.cmdsize as usize;
    }

    println!("Succeeded in decrypting the binary.");
    Ok(())
}
