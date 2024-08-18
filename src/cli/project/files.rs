use crate::cli::project::options::ProjectOptions;
use crate::utils::errors::ProjectError;
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

    // Afficher le contenu de `options` pour le débogage
    println!("Options: {:?}", options);

    let context = tera::Context::from_serialize(options)?;

    // Afficher le contexte pour le débogage
    println!("Contexte de rendu: {:?}", context);

    for (filename, _) in tera.templates.iter() {
        println!("Rendering template: {}", filename);
        let content = tera.render(filename, &context).map_err(|e| {
            ProjectError::TemplateError(
                format!("Failed to render '{}': {}\nContext: {:?}", filename, e, context).into()
            )
        })?;
        std::fs::write(filename, content).map_err(|e| {
            ProjectError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to write file '{}': {}", filename, e),
            ))
        })?;
    }

    Ok(())
}
