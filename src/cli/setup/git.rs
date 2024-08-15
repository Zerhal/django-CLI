// src/cli/setup/git.rs
use std::process::Command;
use std::fs;
use tera::{Tera, Context};

pub fn init() -> Result<(), std::io::Error> {
    println!("Initialisation du dépôt Git...");
    Command::new("git")
        .arg("init")
        .current_dir(".")
        .output()?;
    println!("Dépôt Git initialisé.");
    Ok(())
}

pub fn create_gitignore(venv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Création du fichier .gitignore...");

    // Inclure le contenu du template directement dans le binaire
    let template_content = include_str!("../../../templates/setup/.gitignore.tpl");

    // Créer un objet Tera à partir du contenu du template
    let mut tera = Tera::default();
    tera.add_raw_template("gitignore", template_content)?;
    
    let mut context = Context::new();
    context.insert("venv_name", venv_name);
    
    let gitignore_content = tera.render("gitignore", &context)?;

    fs::write(".gitignore", gitignore_content)?;
    println!(".gitignore créé avec succès.");
    Ok(())
}