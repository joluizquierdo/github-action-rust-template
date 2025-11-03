use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    action_name: String,
    description: String,
    author: String,
    rust: Rust,
    github: Github,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rust {
    name: String,
    edition: String,
    version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Github {
    runner: String,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let mut config = Config::serialize(path);
        config.extract_repo_name_and_owner();

        config
    }

    fn serialize(path: &str) -> Self {
        let config_str = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read config file: {}", path));
        serde_saphyr::from_str(&config_str).expect("Failed to parse config JSON")
    }

    fn extract_repo_name_and_owner(&mut self) {
        let command = "git";
        let command_args = ["remote"];
        let err_msg: &str = "Failed to get git remotes or git is not installed,\
please ensure git is installed and the current directory is a\
git repository with a remote set up.";
        let output = Command::new(command)
            .args(command_args)
            .output()
            .expect(err_msg);

        if output.status.success() {
            let command_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !command_output.contains("origin") {
                panic!(
                    "No 'origin' remote found. Please ensure the current directory is a git repository with an 'origin' remote set up."
                );
            }
        } else {
            panic!(
                "Git command failed with status: {}",
                output.status.code().unwrap_or(-1)
            );
        }

        let command = "git";
        let command_args = ["remote", "get-url", "origin"];
        let err_msg: &str = "Failed to get git remote origin URL";
        let output = Command::new(command)
            .args(command_args)
            .output()
            .expect(err_msg);

        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let repo_name = url.rsplit('/').next().and_then(|s| s.strip_suffix(".git"));
            self.repository_name = match repo_name {
                Some(name) => Some(name.to_string()),
                None => panic!("Failed to extract repository name from URL: {}", url),
            };
            let owner = url.split('/').next().and_then(|s| s.rsplit(':').next());
            self.repository_owner = match owner {
                Some(owner) => Some(owner.to_string()),
                None => panic!("Failed to extract author name from URL: {}", url),
            };
        } else {
            panic!(
                "{}, status code: '{}'",
                err_msg,
                output.status.code().unwrap_or(-1)
            );
        }
    }
}
