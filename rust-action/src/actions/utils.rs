pub fn set_secret(secret_value: &str) {
    println!("::add-mask::{secret_value}");
}

pub fn log_notice(message: &str) {
    println!("::notice::{message}");
}
