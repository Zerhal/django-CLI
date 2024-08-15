use std::process::Command;

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration de Poetry...");
    let pip_cmd = if cfg!(target_os = "windows") { "pip" } else { "pip3" };

    let output = Command::new(pip_cmd)
        .args(&["install", "poetry"])
        .output()?;

    if output.status.success() {
        println!("Poetry installé avec succès.");

        // Vérification de la disponibilité de Poetry
        let poetry_output = Command::new("poetry")
            .arg("--version")
            .output()?;
        
        if poetry_output.status.success() {
            println!("Poetry version : {}", String::from_utf8_lossy(&poetry_output.stdout));

            // Initialiser un projet Poetry sans interaction
            let init_output = Command::new("poetry")
                .args(&["init", "--no-interaction", "--name", "myproject"])
                .output()?;

            if init_output.status.success() {
                println!("Projet configuré avec Poetry.");
                Ok(())
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Échec de l'initialisation du projet avec Poetry."
                )))
            }
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Poetry n'est pas accessible après l'installation."
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Échec de l'installation de Poetry."
        )))
    }
}
