use std::{ffi::OsString, path::PathBuf, str::FromStr};

struct ProjectTemplatesWriter {
    buffer: String,
}

impl ProjectTemplatesWriter {
    pub fn new() -> Self {
        Self {
            buffer: r#"// THIS CODE IS AUTO-GENERATED. DO NOT EDIT. 
// Or, I dunno, you can if you want. Not like I can stop you really.
        
use std::collections::HashMap;
   
pub fn load_project_templates() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut result = HashMap::new();
"#
            .to_string(),
        }
    }

    pub fn finish(mut self) -> String {
        self.buffer.push_str("    result\n}\n");
        self.buffer
    }

    pub fn start_project_definition(&mut self) {
        self.buffer
            .push_str("    let mut project = HashMap::new();\n");
    }

    pub fn finish_project(&mut self, name: &str) {
        self.buffer.push_str("    result.insert(r#\"");
        self.buffer.push_str(name);
        self.buffer.push_str("\"#, project);\n");
    }

    pub fn add_project_file(&mut self, file_name: String, contents: &str) {
        // TODO: make this part less copy-pastey
        let file_name = file_name.replace(std::path::MAIN_SEPARATOR, "/");
        let file_name = file_name.replace("CAMEL_CASE_NAME", "{{camel_case_name}}");
        let file_name = file_name.replace("KEBAB_CASE_NAME", "{{kebab_case_name}}");
        let file_name = file_name.replace("PASCAL_CASE_NAME", "{{pascal_case_name}}");
        let file_name = file_name.replace("SCREAM_CASE_NAME", "{{scream_case_name}}");
        let file_name = file_name.replace("SNAKE_CASE_NAME", "{{snake_case_name}}");
        let file_name = file_name.replace("NAME", "{{name}}");
        let file_name = if file_name.ends_with(".tmpl") {
            &file_name[..file_name.len() - 5]
        } else {
            &file_name
        };
        self.buffer.push_str("    project.insert(r#\"");
        self.buffer.push_str(file_name);
        self.buffer.push_str("\"#, r#\"");
        self.buffer.push_str(contents);
        self.buffer.push_str("\"#);\n");
    }
}

fn iterate_project_root_dir(writer: &mut ProjectTemplatesWriter, root: PathBuf) {
    let entries = match std::fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Could not find root project directory {:?}: {}", root, err);
            std::process::exit(1);
        }
    };
    for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                let proj_name = entry.file_name().to_string_lossy().to_string();
                writer.start_project_definition();
                let mut dir = root.clone();
                dir.push(&proj_name);
                iterate_project(writer, dir, None);
                writer.finish_project(&proj_name);
            }
        }
    }
}

fn iterate_project(
    writer: &mut ProjectTemplatesWriter,
    root: PathBuf,
    file_name_prefix: Option<OsString>,
) {
    let entries = match std::fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Could not find directory {:?}: {}", root, err);
            std::process::exit(1);
        }
    };
    for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                let mut root = root.clone();
                root.push(entry.file_name());
                iterate_project(writer, root, Some(entry.file_name()));
            } else {
                let mut path = root.clone();
                let file_name = if let Some(prefix) = &file_name_prefix {
                    eprintln!("prefix = {:?}", prefix);
                    let mut oss = prefix.clone();
                    let sep = std::path::MAIN_SEPARATOR.to_string();
                    oss.push(&sep);
                    oss.push(entry.file_name());
                    eprintln!("final oss = {:?}", oss);
                    oss
                } else {
                    entry.file_name()
                };
                path.push(entry.file_name());
                let contents = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!(
                            "could not read file {:?}: {} poop={:?} root={:?}",
                            path, e, file_name_prefix, root
                        );
                        std::process::exit(1);
                    }
                };
                writer.add_project_file(file_name.to_string_lossy().to_string(), &contents)
            }
        }
    }
}

fn main() {
    let mut writer = ProjectTemplatesWriter::new();
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=projects");
    let cargo_dir = match PathBuf::from_str(std::env!("CARGO_MANIFEST_DIR")) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Cannot find CARGO_MANIFEST_DIR!: {}", e);
            std::process::exit(1);
        }
    };

    let mut projects_dir = cargo_dir.clone();
    projects_dir.push("projects");

    let mut generate_file = cargo_dir;
    generate_file.push("src/projects.rs");

    iterate_project_root_dir(&mut writer, projects_dir);

    let code = writer.finish();
    if let Err(e) = std::fs::write(generate_file, code) {
        eprintln!("Error writing generated code: {}", e);
        std::process::exit(1);
    };
}
