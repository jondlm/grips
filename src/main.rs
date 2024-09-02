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
    extensions_to_copy: Vec<String>,
    vars: HashMap<String, String>,
}

fn process_dir(
    source: &Path,
    target: &Path,
    render_extension: &str,
    extensions_to_copy: &Vec<String>,
    vars: &HashMap<String, String>,
    hb: &mut Handlebars,
) -> std::io::Result<usize> {
    let mut counter = 0;
    let mut dirs_to_process = vec![source.to_path_buf()];

    while let Some(current_source) = dirs_to_process.pop() {
        for entry in fs::read_dir(&current_source)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                // Queue the subdirectory for processing
                dirs_to_process.push(path.clone());
            } else if file_type.is_file() {
                if let Some(p) = path.to_str() {
                    let ext = match p.find(".") {
                        Some(i) => p[i..].trim_start_matches("."),
                        None => continue,
                    };

                    // Determine the relative path to the source directory
                    let relative_path = path.strip_prefix(source).unwrap();
                    let target_file_path = target.join(relative_path);
                    let target_file_path_str = target_file_path.to_str().unwrap();

                    let do_render = ext == render_extension;
                    let do_copy = extensions_to_copy.contains(&ext.to_string());

                    if do_render || do_copy {
                        // Ensure target directory exists
                        if let Some(parent) = target_file_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                    }

                    if do_render {
                        // Render and write the template
                        let template = fs::read_to_string(&path)?;
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

                        fs::write(target_file_path_str.replace(".hbs.", "."), rendered)?;
                        counter += 1;
                    } else if do_copy {
                        fs::copy(path, target_file_path)?;
                        counter += 1;
                    }
                }
            }
        }
    }

    Ok(counter)
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

    match process_dir(
        &source,
        &target,
        "hbs.html",
        &config.extensions_to_copy,
        &config.vars,
        &mut hb,
    ) {
        Ok(counter) => println!("processed {} files", counter),
        Err(e) => {
            eprintln!("{e}");
            return Err(Box::new(e));
        }
    };

    Ok(())
}
