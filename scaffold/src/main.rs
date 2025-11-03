mod utils;
use minijinja::Environment;
use utils::config::Config;
use utils::template::Template;
use utils::utils::{join_path, remove_no_longer_required_files};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = match std::env::current_dir()?.to_str() {
        Some(path) => path.to_string(),
        None => {
            return Err("Failed to convert path to string".into());
        }
    };

    let config = Config::new(&join_path(&current_dir, "config.yaml"));

    let mut env = Environment::new();
    env.set_trim_blocks(true);

    let templates = vec![
        Template::new(
            "action",
            &join_path(&current_dir, "templates/action.yml.j2"),
            &join_path(&current_dir, "action.yml"),
        ),
        Template::new(
            "readme",
            &join_path(&current_dir, "templates/README.md.j2"),
            &join_path(&current_dir, "README.md"),
        ),
        Template::new(
            "test-workflow",
            &join_path(&current_dir, "templates/test-workflow.yml.j2"),
            &join_path(&current_dir, ".github/workflows/test.yml"),
        ),
        Template::new(
            "release-workflow",
            &join_path(&current_dir, "templates/release-workflow.yml.j2"),
            &join_path(&current_dir, ".github/workflows/release.yml"),
        ),
    ];

    Template::read_templates(&templates, &mut env);
    Template::write_rendered_templates(&templates, &env, &config);

    let no_longer_required_files = vec![
        join_path(&current_dir, "templates"),
        join_path(&current_dir, "config.yaml"),
        join_path(&current_dir, "scaffold"),
    ];
    remove_no_longer_required_files(no_longer_required_files.as_slice());

    Ok(())
}
