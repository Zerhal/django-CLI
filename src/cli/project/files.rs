use crate::utils::errors::ProjectError;
use crate::cli::project::options::ProjectOptions;
use tera::Tera;

pub async fn generate_project_files(options: &ProjectOptions) -> Result<(), ProjectError> {
    println!("Generating project files...");
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            ".gitignore",
            include_str!("../../../templates/setup/.gitignore.tpl"),
        ),
        (
            "Dockerfile",
            include_str!("../../../templates/setup/Dockerfile.tpl"),
        ),
        (
            "README.md",
            include_str!("../../../templates/setup/README.md.tpl"),
        ),
        (
            "project_structure.yml",
            include_str!("../../../templates/project/project_structure.tpl"),
        ),
    ])?;

    let context = tera::Context::from_serialize(options)?;

    for (filename, _) in tera.templates.iter() {
        let content = tera.render(filename, &context)?;
        std::fs::write(filename, content)?;
    }

    Ok(())
}