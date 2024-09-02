use handlebars::Handlebars;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    source: String,
    target: String,
    vars: HashMap<String, String>,
}

fn process_dir(
    source: &Path,
    target: &Path,
    extension: &str,
    vars: &HashMap<String, String>,
    hb: &mut Handlebars,
) -> std::io::Result<()> {
    if source.is_dir() {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                // Recursively copy files in subdirectories
                let target_subdir = target.join(entry.file_name());
                fs::create_dir_all(&target_subdir)?;
                process_dir(&path, &target_subdir, extension, vars, hb)?;
            } else if file_type.is_file() {
                if let Some(p) = path.to_str() {
                    let ext = match p.rfind(extension) {
                        Some(i) => &p[i..],
                        None => continue,
                    };
                    if ext == extension {
                        // Determine the relative path to the source directory
                        let relative_path = path.strip_prefix(source).unwrap();
                        let target_file_path = target.join(relative_path);

                        // Ensure target directory exists
                        if let Some(parent) = target_file_path.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        // Render and write the template
                        let template = fs::read_to_string(entry.path())?;
                        let rendered_result = hb.render_template(&template, &vars);

                        let rendered = match rendered_result {
                            Ok(r) => r,
                            Err(_) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "render error",
                                ))
                            }
                        };

                        fs::write(target_file_path, rendered)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let grips_config_path = Path::new("grips.json");

    if !grips_config_path.exists() {
        panic!("Missing grips.json config file in cwd");
    }

    let config_string = fs::read_to_string(grips_config_path)?;
    let config: Config = serde_json::from_str(&config_string)?;
    let source = Path::new(&config.source);
    let target = Path::new(&config.target);
    let mut hb = Handlebars::new();

    process_dir(&source, &target, ".hbs.html", &config.vars, &mut hb)?;

    println!("done");

    Ok(())
}
