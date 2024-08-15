// src/cli/setup/env.rs
use crate::utils::tools::get_compatible_django_version;
use std::process::Command;
use dialoguer::Confirm;
use tera::Tera;

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
        println!("Ou visitez https://www.python.org/downloads/mac-osx/ pour une installation manuelle.");
    } else if cfg!(target_os = "linux") {
        println!("Utilisez le gestionnaire de paquets de votre distribution pour installer Python.");
        println!("Par exemple, sur Ubuntu ou Debian : sudo apt-get install python3");
    } else {
        println!("Visitez https://www.python.org/downloads/ pour télécharger Python pour votre système.");
    }

    let _ = Confirm::new().with_prompt("Appuyez sur Entrée une fois Python installé").interact()?;
    check_and_install_python()
}

pub fn check_and_install_pip(python_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pip_command = if python_version.starts_with("3") { "pip3" } else { "pip" };

    match Command::new(pip_command).arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("pip est installé. ({})", String::from_utf8_lossy(&output.stdout).trim());
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
                Command::new("curl")
                    .args(&["-O", get_pip_url])
                    .status()?;

                // Exécuter get-pip.py
                let python_cmd = if python_version.starts_with("3") { "python3" } else { "python" };
                Command::new(python_cmd)
                    .arg("get-pip.py")
                    .status()?;

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

pub fn check_django_version(venv_name: &str, required_version: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let python_path = if cfg!(target_os = "windows") {
        format!("{}\\Scripts\\python", venv_name)
    } else {
        format!("{}/bin/python", venv_name)
    };

    let output = Command::new(python_path)
        .arg("-m")
        .arg("django")
        .arg("--version")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let installed_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if installed_version.starts_with(required_version) {
                println!("Django {} est déjà installé.", installed_version);
                Ok(true)
            } else {
                println!("Version de Django installée ({}) incompatible avec la version requise ({}).", installed_version, required_version);
                Ok(false)
            }
        }
        _ => {
            println!("Django n'est pas installé.");
            Ok(false)
        }
    }
}

pub fn create_virtual_env(venv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Création de l'environnement virtuel nommé '{}'...", venv_name);

    let python_cmd = if cfg!(target_os = "windows") { "python" } else { "python3" };

    let output = Command::new(python_cmd)
        .args(&["-m", "venv", venv_name])
        .output()?;

    if output.status.success() {
        println!("Environnement virtuel créé avec succès.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Échec de la création de l'environnement virtuel."
        )))
    }
}

pub fn install_dependencies(venv_name: &str, python_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Vérification de la version de Django...");

    if let Some(django_version) = get_compatible_django_version(python_version) {
        if check_django_version(venv_name, django_version)? {
            println!("Aucune installation de Django nécessaire.");
            return Ok(());
        }

        println!("Installation de Django version {}...", django_version);

        let pip_path = if cfg!(target_os = "windows") {
            format!("{}\\Scripts\\pip", venv_name)
        } else {
            format!("{}/bin/pip", venv_name)
        };

        let output = Command::new(pip_path)
            .args(&["install", &format!("django=={}", django_version)])
            .output()?;

        if output.status.success() {
            println!("Django {} installé avec succès.", django_version);
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Échec de l'installation des dépendances."
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Aucune version compatible de Django trouvée pour cette version de Python."
        )))
    }
}

pub fn create_readme(python_version: &str, venv_name: &str, django_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Création du fichier README.md...");
    
    
    let template_content = include_str!("../../../templates/setup/README.md.tpl");
    // Créez un objet Tera à partir de la chaîne de caractères
    let mut tera = Tera::default();
    tera.add_raw_template("README", template_content)?;
    
    let mut context = tera::Context::new();
    context.insert("python_version", python_version);
    context.insert("venv_name", venv_name);
    context.insert("django_version", django_version); // Add django_version to the context
    
    let readme_content = tera.render("README", &context)?;

    std::fs::write("README.md", readme_content)?;
    println!("README.md créé avec succès.");
    Ok(())
}

