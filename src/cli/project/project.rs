// src/cli/project/project.rs
use crate::utils::tools::get_compatible_django_version;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::error::Error;
use std::process::Command;
use tera::Tera;

pub fn create_project() -> Result<(), Box<dyn Error>> {
    println!("Creating a new Django project...");

    // Vérification des prérequis
    let python_version = check_and_install_python()?;
    check_and_install_pip(&python_version)?;
    let django_version = get_compatible_django_version(&python_version)
        .ok_or("No compatible Django version found")?;

    println!("Detected Python version: {}", python_version);
    println!("Recommended Django version: {}", django_version);

    // Confirmation pour continuer
    if !Confirm::new()
        .with_prompt("Do you want to continue with the project creation?")
        .interact()?
    {
        println!("Project creation canceled.");
        return Ok(());
    }

    // CLI interactive pour la création du projet
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact_text()?;

    let project_types = vec!["rest", "graphql", "fullstack"];
    let project_type = Select::new()
        .with_prompt("Select the project type")
        .items(&project_types)
        .interact()?;

    let frameworks = vec!["react", "vue", "angular"];
    let frontend_framework = if project_types[project_type] == "fullstack" {
        Some(
            Select::new()
                .with_prompt("Select the frontend framework")
                .items(&frameworks)
                .interact()?,
        )
    } else {
        None
    };

    let db_types = vec!["sqlite", "postgres", "mysql"];
    let db_type = Select::new()
        .with_prompt("Select the database type")
        .items(&db_types)
        .interact()?;

    let auth_systems = vec!["django", "jwt", "oauth"];
    let auth_system = Select::new()
        .with_prompt("Select the authentication system")
        .items(&auth_systems)
        .interact()?;

    let package_managers = vec!["Venv", "Poetry", "None"];
    let package_manager = Select::new()
        .with_prompt("Select the package manager")
        .items(&package_managers)
        .interact()?;

    let options = vec!["Git", "Docker", "Readme"];
    let selections = MultiSelect::new()
        .with_prompt("Select additional options")
        .items(&options)
        .interact()?;

    // Création du projet
    println!("Creating project structure...");
    // TODO: Implémenter la création de la structure du projet

    // Utilisation de Tera pour générer les fichiers
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

    let mut context = tera::Context::new();
    context.insert("project_name", &project_name);
    context.insert("project_type", project_types[project_type]);
    context.insert("db_type", db_types[db_type]);
    context.insert("auth_system", auth_systems[auth_system]);

    // TODO: Générer les fichiers avec Tera

    // Configuration de settings.py
    println!("Configuring settings.py...");
    // TODO: Implémenter la configuration de settings.py

    println!("You selected: {}", package_managers[package_manager]);

    // Initialisation de Git si sélectionné
    if selections.contains(&0) {
        println!("Initializing Git repository...");
        Command::new("git").arg("init").output()?;
    }

    // Configuration du frontend si fullstack
    if let Some(framework) = frontend_framework {
        println!("Configuring frontend with {}...", frameworks[framework]);
        // TODO: Implémenter la configuration du frontend
    }

    println!("Project '{}' has been created successfully!", project_name);
    Ok(())
}

pub fn check_and_install_python() -> Result<String, Box<dyn std::error::Error>> {
    let python_commands = ["python", "python3", "py"];

    for cmd in python_commands.iter() {
        match Command::new(cmd).arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap_or("Unknown")
                    .to_string();
                return Ok(version);
            }
            _ => continue,
        }
    }

    // Python n'est pas installé
    println!("Python n'est pas installé. Veuillez l'installer manuellement.");

    if cfg!(target_os = "windows") {
        println!("Visitez https://www.python.org/downloads/windows/ pour télécharger Python pour Windows.");
    } else if cfg!(target_os = "macos") {
        println!("Vous pouvez installer Python via Homebrew en exécutant : brew install python");
        println!(
            "Ou visitez https://www.python.org/downloads/mac-osx/ pour une installation manuelle."
        );
    } else if cfg!(target_os = "linux") {
        println!(
            "Utilisez le gestionnaire de paquets de votre distribution pour installer Python."
        );
        println!("Par exemple, sur Ubuntu ou Debian : sudo apt-get install python3");
    } else {
        println!(
            "Visitez https://www.python.org/downloads/ pour télécharger Python pour votre système."
        );
    }

    let _ = Confirm::new()
        .with_prompt("Appuyez sur Entrée une fois Python installé")
        .interact()?;
    check_and_install_python()
}

pub fn check_and_install_pip(python_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pip_command = if python_version.starts_with("3") {
        "pip3"
    } else {
        "pip"
    };

    match Command::new(pip_command).arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!(
                "pip est installé. ({})",
                String::from_utf8_lossy(&output.stdout).trim()
            );
            return Ok(());
        }
        _ => {
            println!("pip n'est pas installé. Installation en cours...");

            if cfg!(target_os = "windows") {
                // Sur Windows, pip est généralement inclus avec Python
                println!("pip devrait être inclus avec votre installation Python.");
                println!("Si ce n'est pas le cas, essayez de réinstaller Python en cochant l'option 'Add Python to PATH'.");
            } else {
                // Sur macOS et Linux, on peut utiliser get-pip.py
                let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";

                // Télécharger get-pip.py
                Command::new("curl").args(&["-O", get_pip_url]).status()?;

                // Exécuter get-pip.py
                let python_cmd = if python_version.starts_with("3") {
                    "python3"
                } else {
                    "python"
                };
                Command::new(python_cmd).arg("get-pip.py").status()?;

                // Supprimer get-pip.py
                if cfg!(target_os = "windows") {
                    Command::new("del").arg("get-pip.py").status()?;
                } else {
                    Command::new("rm").arg("get-pip.py").status()?;
                }
            }

            // Vérifier à nouveau l'installation de pip
            match Command::new(pip_command).arg("--version").output() {
                Ok(output) if output.status.success() => {
                    println!("pip a été installé avec succès.");
                    Ok(())
                }
                _ => Err("Échec de l'installation de pip.".into()),
            }
        }
    }
}
