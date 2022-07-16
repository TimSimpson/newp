use crate::projects;

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
