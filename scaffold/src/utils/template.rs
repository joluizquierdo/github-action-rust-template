use super::config::Config;

use minijinja::Environment;
#[derive(Debug)]
pub struct Template {
    name: String,
    content: String,
    rendered_path: String,
}

impl Template {
    pub fn new(name: &str, src_path: &str, rendered_path: &str) -> Self {
        let content = std::fs::read_to_string(src_path)
            .unwrap_or_else(|_| panic!("Failed to read template file: {}", src_path));

        Template {
            name: name.to_string(),
            content,
            rendered_path: rendered_path.to_string(),
        }
    }

    pub fn read_templates<'a>(templates: &'a [Template], env: &mut Environment<'a>) {
        for t in templates {
            println!("Reading template: {}", t.name);
            env.add_template(&t.name, &t.content)
                .expect("Failed to add template to environment");
        }
    }

    pub fn write_rendered_templates(templates: &[Template], env: &Environment, config: &Config) {
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
