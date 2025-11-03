use super::utils::set_secret;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub struct GitHubContext {
    pub token: String,
    output_file: File,
}

impl GitHubContext {
    pub fn new() -> Self {
        let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
        set_secret(&token);

        let output_file_path = env::var("GITHUB_OUTPUT").expect("GITHUB_OUTPUT not set");
        let output_file = OpenOptions::new()
            .append(true)
            .create(false)
            .open(output_file_path)
            .expect("Failed to open GITHUB_OUTPUT file");

        Self { token, output_file }
    }

    pub fn set_output(&mut self, name: &str, value: &str) {
        let err_msg = format!("Failed to write output {} to GITHUB_OUTPUT file", name);
        if value.contains('\n') {
            writeln!(self.output_file, "{}<<EOF{}EOF", name, value).expect(&err_msg);
        } else {
            writeln!(self.output_file, "{}={}", name, value).expect(&err_msg);
        }
    }

    // pub fn set_output_secret(&mut self, name: &str, value: &str) {
    //     set_secret(value);
    //     self.set_output(name, value);
    // }
}
