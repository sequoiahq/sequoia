#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![feature(c_size_t)]

use std::ffi::{c_void, CStr, CString};
use std::ops::Drop;
use std::os::raw::c_char;
use std::ptr::null;

use crate::error::MediaInfoError;
use crate::MediaInfoError::OpenFailed;

mod error;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Handle {
    handle: *mut c_void,
    has_file_open: bool,
}

impl Handle {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let handle = unsafe { MediaInfoA_New() as *mut c_void };

        Ok(Self {
            handle,
            has_file_open: false,
        })
    }

    pub fn open<P: Into<Vec<u8>>>(&mut self, path: P) -> Result<(), MediaInfoError> {
        let path = CString::new(path)?;

        if self.has_file_open {
            unsafe {
                MediaInfoA_Close(self.handle);
            }
            self.has_file_open = false;
        }

        let filepath_string = path.into_raw();
        let success = unsafe {
            let size =
                MediaInfoA_Open(self.handle.as_mut().ok_or(OpenFailed)?, filepath_string) as usize;
            let _ = CString::from_raw(filepath_string);
            size
        };

        if success == 0 {
            return Err(OpenFailed);
        }
        self.has_file_open = true;

        Ok(())
    }

    pub fn inform(&self) -> Result<&str, MediaInfoError> {
        unsafe {
            let informhandle = MediaInfoA_Inform(
                self.handle,
                MediaInfo_infooptions_t_MediaInfo_InfoOption_Max.into(),
            ) as *const c_char;
            CStr::from_ptr(informhandle)
                .to_str()
                .map_err(MediaInfoError::Utf8Error)
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            if self.has_file_open {
                MediaInfoA_Close(self.handle);
            }
            MediaInfoA_Delete(self.handle);
        }
    }
}

struct ListHandle {
    handle: *mut c_void,
    file_count: u32,
}

impl ListHandle {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let handle = unsafe { MediaInfoListA_New() as *mut c_void };

        Ok(Self {
            handle,
            file_count: 0,
        })
    }

    pub fn open<P: Into<Vec<u8>>>(&mut self, path: P) -> Result<(), MediaInfoError> {
        let path = CString::new(path)?;

        let filepath_string = path.into_raw();
        let success = unsafe {
            let size = MediaInfoListA_Open(
                self.handle.as_mut().ok_or(OpenFailed)?,
                filepath_string,
                MediaInfo_fileoptions_t_MediaInfo_FileOption_Max,
            ) as usize;
            let _ = CString::from_raw(filepath_string);
            size
        };

        if success == 0 {
            return Err(OpenFailed);
        }
        self.file_count += 1;

        Ok(())
    }

    pub fn inform(&self, file_index: u32) -> Result<&str, MediaInfoError> {
        unsafe {
            let informhandle = MediaInfoListA_Inform(
                self.handle,
                file_index as size_t,
                MediaInfo_infooptions_t_MediaInfo_InfoOption_Max.into(),
            ) as *const c_char;
            CStr::from_ptr(informhandle)
                .to_str()
                .map_err(MediaInfoError::Utf8Error)
        }
    }

    pub fn inform_all(&self) -> Result<String, MediaInfoError> {
        let mut informed = vec![];
        for i in 0..self.file_count {
            informed.push(self.inform(i)?);
        }

        Ok(informed.join("\n"))
    }

    pub fn count(&self) -> u32 {
        return self.file_count;
    }
}

impl Drop for ListHandle {
    fn drop(&mut self) {
        unsafe {
            MediaInfoListA_Delete(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Handle, ListHandle};

    #[test]
    fn test_single() {
        println!("Starting");
        let mut handle = Handle::new().unwrap();
        for i in 0..1 {
            if i % 100 == 0 {
                println!("iteration: {}", i);
            }
            match handle.open("media/moonshine.mp4") {
                Ok(()) => {}
                Err(err) => {
                    println!("Err Moonshine: {:?}", err);
                }
            }

            match handle.open("media/avatarfinale.mkv") {
                Ok(()) => {
                    // println!("{}", handle.inform().unwrap());
                }
                Err(err) => {
                    println!("Err Avatar: {:?}", err);
                }
            }

            match handle.open("media/notthere.mkv") {
                Ok(()) => {}
                Err(err) => {
                    println!("Err NotThere: {:?}", err);
                }
            }
        }
        println!("Finished");
    }

    #[test]
    fn test_list() {
        println!("Starting");
        let mut handle = ListHandle::new().unwrap();
        for i in 0..1 {
            if i % 100 == 0 {
                println!("iteration: {}", i);
            }
            match handle.open("media/moonshine.mp4") {
                Ok(()) => {}
                Err(err) => {
                    println!("Err Moonshine: {:?}", err);
                }
            }

            match handle.open("media/avatarfinale.mkv") {
                Ok(()) => {}
                Err(err) => {
                    println!("Err Avatar: {:?}", err);
                }
            }

            println!(
                "======\nInform all:\n\n{}\n======",
                handle.inform_all().unwrap()
            );

            match handle.open("media/notthere.mkv") {
                Ok(()) => {}
                Err(err) => {
                    println!("Err NotThere: {:?}", err);
                }
            }
        }
        println!("Finished");
    }
}
