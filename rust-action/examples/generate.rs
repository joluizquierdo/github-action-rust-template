use minijinja::Environment;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

fn main() {
    let config = Config::new("../config.yaml");

    let mut env = Environment::new();
    env.set_trim_blocks(true);

    let templates = vec![
        Template::new("action", "../templates/action.yml.j2", "../action.yml"),
        Template::new("readme", "../templates/README.md.j2", "../README.md"),
        Template::new("cargo", "../templates/cargo.toml.j2", "Cargo.toml"),
        Template::new(
            "test-action",
            "../templates/test-action.yml.j2",
            &join_root_path(".github/workflows/test_action.yml"),
        ),
    ];

    Template::read_templates(&templates, &mut env);
    Template::write_rendered_templates(&templates, &env, &config);
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    action_name: String,
    description: String,
    author: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    rust: Rust,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repository_owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    name: String,
    description: String,
    required: bool,
    default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    name: String,
    description: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Rust {
    name: String,
    edition: String,
    version: String,
}

impl Config {
    fn new(path: &str) -> Self {
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

#[derive(Debug)]
struct Template {
    name: String,
    content: String,
    rendered_path: String,
}

impl Template {
    fn new(name: &str, src_path: &str, rendered_path: &str) -> Self {
        let content = std::fs::read_to_string(src_path)
            .unwrap_or_else(|_| panic!("Failed to read template file: {}", src_path));

        Template {
            name: name.to_string(),
            content,
            rendered_path: rendered_path.to_string(),
        }
    }

    fn read_templates<'a>(templates: &'a [Template], env: &mut Environment<'a>) {
        for t in templates {
            println!("Reading template: {}", t.name);
            env.add_template(&t.name, &t.content)
                .expect("Failed to add template to environment");
        }
    }

    fn write_rendered_templates(templates: &[Template], env: &Environment, config: &Config) {
        for t in templates {
            let tmpl = env
                .get_template(&t.name)
                .expect("Failed to get template from environment");

            println!("Rendering template: {}", t.name);
            let rendered = tmpl.render(config).expect("Failed to render template");

            println!("Writing rendered template to: {}", t.rendered_path);

            let parent = std::path::Path::new(&t.rendered_path).parent();
            if let Some(p) = parent {
                std::fs::create_dir_all(p).unwrap_or_else(|_| {
                    panic!("Failed to create directories for path: {}", p.display())
                });
            }
            std::fs::write(&t.rendered_path, &rendered).unwrap_or_else(|_| {
                panic!(
                    "Failed to write rendered template to file: {}",
                    t.rendered_path
                )
            });
        }
    }
}

fn join_root_path(relative_path: &str) -> String {
    let repository_root = get_repository_root_path();
    let err_msg = &format!(
        "Failed to join repository root path '{}' with relative path '{}'",
        repository_root, relative_path
    );

    Path::new(&repository_root)
        .join(relative_path)
        .to_str()
        .expect(err_msg)
        .to_string()
}

fn get_repository_root_path() -> String {
    let command = "git";
    let command_args = ["rev-parse", "--show-toplevel"];
    let err_msg: &str = "Failed to get git repository root path,\
please ensure git is installed and the current directory is a\
git repository.";
    let output = Command::new(command)
        .args(command_args)
        .output()
        .expect(err_msg);

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        panic!(
            "Git command failed with status: {}",
            output.status.code().unwrap_or(-1)
        );
    }
}
