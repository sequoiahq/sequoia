use aes::Aes128;
use aes::cipher::{BlockCipher, BlockEncrypt, BlockDecrypt};
use std::cmp;

fn main() {
    let processing_key = [
        0x09, 0xF9, 0x11, 0x02, 0x9D, 0x74, 0xE3, 0x5B, 0xD8, 0x41, 0x56, 0xC5, 0x63, 0x56, 0x88, 0xC0,
    ];

    let encrypted_c_value = [
        0x6D, 0x02, 0xCA, 0xC6, 0x7B, 0x1A, 0x7E, 0x95, 0xC2, 0x16, 0xEF, 0xD4, 0xC9, 0x28, 0x09, 0xCF,
    ];

    let mut decrypted_c_value = [0u8; 16];
  
    let uv = [0x00, 0x00, 0x00, 0x01];

    let mut media_key = [0u8; 16];

    // encrypted payload from the king kong dvd
    let encrypted_verification_data = [
        0x87, 0xB8, 0xA2, 0xB7, 0xC1, 0x0B, 0x9F, 0xAD, 0xF8, 0xC4, 0x36, 0x1E, 0x23, 0x86, 0x59, 0xE5,
    ];

    let decrypted_verification_data_should_be = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut decrypted_verification_data = [0u8; 16];

    let volume_id = [
        0x40, 0x00, 0x09, 0x18, 0x20, 0x06, 0x08, 0x41, 0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,
    ];

    let mut decrypted_volumeid = [0u8; 16];

    let mut volume_unique_key = [0u8; 16];

    // decrypt c-value with proccessing key
    let cipher = Aes128::new(&processing_key);
    cipher.decrypt_block(&encrypted_c_value, &mut decrypted_c_value);

    // xor it with uv of c-value
    for j in 0..16 {
        if j < 12 {
            media_key[j] = decrypted_c_value[j];
        } else {
            media_key[j] = decrypted_c_value[j] ^ uv[j - 12];
        }
    }

    // check if key is correct using verify media key record
    let cipher = Aes128::new(&media_key);
    cipher.decrypt_block(&encrypted_verification_data, &mut decrypted_verification_data);

    if decrypted_verification_data.iter().zip(&decrypted_verification_data_should_be).all(|(a, b)| a == b) {
        println!("Decrypted Verification Data: ");
        for j in 0..16 {
            print!("{:02X} ", decrypted_verification_data[j]);
        }
        println!();
    }

    // AES-G (decrypt and XOR) on media key + volumeID
    let cipher = Aes128::new(&media_key);
    cipher.decrypt_block(&volume_id, &mut decrypted_volumeid);
    for j in 0..16 {
        volume_unique_key[j] = volume_id[j] ^ decrypted_volumeid[j];
    }
    println!("Volume Unique Key: ");
    for j in 0..16 {
        print!("{:02X} ", volume_unique_key[j]);
    }
    println!();
}
