pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn log_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}
