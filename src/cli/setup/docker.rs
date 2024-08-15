// src/cli/setup/docker.rs
use std::fs;
use tera::{Tera, Context};

pub fn generate_dockerfile(python_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Génération du Dockerfile...");

    // Inclure le contenu du template directement dans le binaire
    let template_content = include_str!("../../../templates/setup/Dockerfile.tpl");
    
    // Créer un objet Tera à partir du contenu du template
    let mut tera = Tera::default();
    tera.add_raw_template("dockerfile", template_content)?;
    
    let mut context = Context::new();
    context.insert("python_version", python_version);
    
    let dockerfile_content = tera.render("dockerfile", &context)?;

    fs::write("Dockerfile", dockerfile_content)?;
    println!("Dockerfile généré avec succès.");
    Ok(())
}
