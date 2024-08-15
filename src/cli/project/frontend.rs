// src/cli/frontend.rs

use std::process::Command;

pub fn configure_frontend(name: &str, frontend: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration du frontend {}...", frontend);
    
    if Command::new("node").arg("--version").output().is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Node.js n'est pas installé. Veuillez l'installer pour configurer le frontend."
        )));
    }

    match frontend {
        "react" => {
            Command::new("npx")
                .args(&["create-react-app", "frontend"])
                .current_dir(name)
                .output()?;
        }
        "vue" => {
            Command::new("npm")
                .args(&["init", "vue@latest", "frontend"])
                .current_dir(name)
                .output()?;
        }
        "angular" => {
            Command::new("ng")
                .args(&["new", "frontend"])
                .current_dir(name)
                .output()?;
        }
        _ => println!("Frontend non reconnu. Aucune configuration effectuée."),
    }
    println!("Frontend configuré.");
    Ok(())
}
