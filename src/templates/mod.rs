use crate::projects;
use convert_case::Case;
use convert_case::Casing;
use std::collections::HashMap;

/// Summary info abotu a template
pub struct TemplateInfo {
    pub name: String,
    pub desc: String,
}

/// Grabs a summary of info about the template
pub fn get_list() -> Vec<TemplateInfo> {
    let projects = projects::load_project_templates();
    let mut v = Vec::new();
    for (name, value) in projects.into_iter() {
        let desc = match value.get("__desc") {
            Some(d) => d.to_string(),
            None => "".to_string(),
        };
        v.push(TemplateInfo {
            name: name.to_string(),
            desc,
        });
    }
    v
}

/// Represents rendered files ready to be written
pub type RenderedContent = HashMap<String, String>;

type ProjectTemplate = HashMap<&'static str, &'static str>;

fn load(name: &str) -> ProjectTemplate {
    let projects = projects::load_project_templates();
    match projects.get(name) {
        None => {
            eprintln!("Cannot find template named \"{}\".", name);
            std::process::exit(1);
        }
        Some(template) => template.clone(),
    }
}

pub struct RenderOptions {
    pub author: String,
    pub description: String,
    pub license: String,
    pub name: String,
    pub r#type: String,
}

fn evaluate(options: &RenderOptions, template: ProjectTemplate) -> RenderedContent {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &options.name);
    ctx.insert("description", &options.description);
    ctx.insert("camel_case_name", &options.name.to_case(Case::Camel));
    ctx.insert("kebab_case_name", &options.name.to_case(Case::Kebab));
    ctx.insert("pascal_case_name", &options.name.to_case(Case::Pascal));
    ctx.insert(
        "scream_case_name",
        &options.name.to_case(Case::ScreamingSnake),
    );
    ctx.insert("snake_case_name", &options.name.to_case(Case::Snake));
    ctx.insert("license", &options.license);
    ctx.insert("author", &options.author);
    let mut result = HashMap::new();
    for (file_name, content) in template.into_iter() {
        if file_name == "__desc" {
            continue;
        }
        let new_file_name = match tera::Tera::one_off(file_name, &ctx, false) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "Error extracting template file name. Original file {}: {}",
                    file_name, e
                );
                std::process::exit(1);
            }
        };
        let new_content = match tera::Tera::one_off(content, &ctx, false) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Error rendering template file {}: {}\nDetails: {:?}",
                    file_name, e, e
                );
                std::process::exit(1);
            }
        };
        result.insert(new_file_name, new_content);
    }
    result
}

/// Renders a template
pub fn render(options: &RenderOptions) -> RenderedContent {
    let template = load(&options.r#type);
    evaluate(options, template)
}
