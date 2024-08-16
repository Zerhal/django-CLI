use crate::cli::project::options::{AdditionalOption, ProjectOptions};
use crate::utils::command::run_command;
use crate::utils::errors::ProjectError;
use dialoguer::{Confirm, Select};
use std::future::Future;
use std::pin::Pin;
use std::process::Command;

pub async fn check_prerequisites() -> Result<(), ProjectError> {
    let python_version = check_and_install_python().await?;
    check_and_install_pip(&python_version).await?;
    check_and_install_django(&python_version).await?;

    if !Confirm::new()
        .with_prompt("Do you want to continue with the project creation?")
        .interact()?
    {
        return Err(ProjectError::UserCancelled);
    }

    Ok(())
}

pub fn check_and_install_python(
) -> Pin<Box<dyn Future<Output = Result<String, ProjectError>> + Send>> {
    Box::pin(async {
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
                Ok(output) => {
                    return Err(ProjectError::CommandFailed(
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ))
                }
                Err(e) => return Err(ProjectError::Io(e)),
            }
        }

        // Python is not installed
        println!("Python n'est pas installé. Veuillez l'installer manuellement.");

        if cfg!(target_os = "windows") {
            println!("Visitez https://www.python.org/downloads/windows/ pour télécharger Python pour Windows.");
        } else if cfg!(target_os = "macos") {
            println!(
                "Vous pouvez installer Python via Homebrew en exécutant : brew install python"
            );
            println!("Ou visitez https://www.python.org/downloads/mac-osx/ pour une installation manuelle.");
        } else if cfg!(target_os = "linux") {
            println!(
                "Utilisez le gestionnaire de paquets de votre distribution pour installer Python."
            );
            println!("Par exemple, sur Ubuntu ou Debian : sudo apt-get install python3");
        } else {
            println!("Visitez https://www.python.org/downloads/ pour télécharger Python pour votre système.");
        }

        // Wait for the user to install Python manually
        Confirm::new()
            .with_prompt("Appuyez sur Entrée une fois Python installé et prêt à être vérifié.")
            .interact()
            .map_err(ProjectError::DialoguerError)?;

        // Retry checking for Python installation
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
                Ok(output) => {
                    return Err(ProjectError::CommandFailed(
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ))
                }
                Err(e) => return Err(ProjectError::Io(e)),
            }
        }

        // If still not installed, return an error
        Err(ProjectError::CommandFailed(
            "Python n'est toujours pas installé.".into(),
        ))
    })
}

pub async fn check_and_install_pip(python_version: &str) -> Result<(), ProjectError> {
    let pip_command = if python_version.starts_with("3") {
        "pip3"
    } else {
        "pip"
    };

    let pip_installed = match Command::new(pip_command).arg("--version").output() {
        Ok(output) if output.status.success() => {
            true
        }
        Ok(output) => {
            println!(
                "pip n'est pas installé correctement : {}",
                String::from_utf8_lossy(&output.stderr).to_string()
            );
            false
        }
        Err(_) => false,
    };

    if !pip_installed {
        println!("pip n'est pas installé. Installation en cours...");

        if cfg!(target_os = "windows") {
            println!("pip devrait être inclus avec votre installation Python.");
            println!(
                "Si ce n'est pas le cas, essayez de réinstaller Python en cochant l'option 'Add Python to PATH'."
            );
        } else {
            let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";
            run_command("curl", &["-O", get_pip_url])
                .await
                .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

            let python_cmd = if python_version.starts_with("3") {
                "python3"
            } else {
                "python"
            };
            run_command(python_cmd, &["get-pip.py"])
                .await
                .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

            let remove_command = if cfg!(target_os = "windows") {
                "del"
            } else {
                "rm"
            };
            run_command(remove_command, &["get-pip.py"])
                .await
                .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;
        }

        match Command::new(pip_command).arg("--version").output() {
            Ok(output) if output.status.success() => {
                println!("pip a été installé avec succès.");
                Ok(())
            }
            Ok(output) => Err(ProjectError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )),
            Err(e) => Err(ProjectError::Io(e)),
        }
    } else {
        Ok(())
    }
}

pub async fn check_and_install_django(python_version: &str) -> Result<(), ProjectError> {
    // Normaliser la version de Python pour utiliser uniquement les numéros de version majeurs et mineurs
    let normalized_version = python_version
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");

    // Obtenir les versions compatibles de Django en fonction de la version de Python
    let compatible_versions: Vec<&str> = {
        let compatibility = vec![
            ("5.0", vec!["3.10", "3.11", "3.12"]),
            ("4.2", vec!["3.8", "3.9", "3.10", "3.11", "3.12"]),
            ("4.1", vec!["3.8", "3.9", "3.10", "3.11"]),
            ("4.0", vec!["3.8", "3.9", "3.10"]),
            ("3.2", vec!["3.6", "3.7", "3.8", "3.9", "3.10"]),
        ];

        compatibility
            .iter()
            .filter_map(|(django_version, python_versions)| {
                if python_versions.contains(&normalized_version.as_str()) {
                    Some(*django_version)
                } else {
                    None
                }
            })
            .collect()
    };

    if compatible_versions.is_empty() {
        println!("Aucune version compatible de Django n'a été trouvée pour Python {}.", python_version);
        return Err(ProjectError::UnsupportedFramework(format!(
            "Python {} n'est pas compatible avec les versions de Django supportées.",
            python_version
        )));
    }

    // Vérifier si Django est déjà installé
    let django_installed = match Command::new("python")
        .arg("-m")
        .arg("django")
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            let installed_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("Django version {} est déjà installé.", installed_version);
            // Vérifier la compatibilité de la version installée
            compatible_versions.iter().any(|&version| installed_version.starts_with(version))
        }
        Ok(_) | Err(_) => false,
    };

    if django_installed {
        println!("La version de Django installée est compatible avec Python {}.", python_version);
        return Ok(());
    }

    // Django n'est pas installé ou la version installée n'est pas compatible
    println!("Django n'est pas installé ou la version installée n'est pas compatible.");

    // Proposer à l'utilisateur de choisir la version de Django à installer parmi les versions compatibles
    let selection = Select::new()
        .with_prompt("Sélectionnez la version de Django à installer")
        .items(&compatible_versions)
        .default(0)
        .interact()
        .map_err(ProjectError::DialoguerError)?;

    let selected_version = compatible_versions[selection];

    println!("Installation de Django version {}...", selected_version);

    // Installer la version sélectionnée de Django
    let install_command = format!("{} -m pip install django=={}", python_version, selected_version);
    run_command("sh", &["-c", &install_command])
        .await
        .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

    println!("Django version {} a été installé avec succès.", selected_version);

    Ok(())
}


pub async fn initialize_version_control(options: &ProjectOptions) -> Result<(), ProjectError> {
    if options.additional_options.contains(&AdditionalOption::Git) {
        println!("Initializing Git repository...");
        run_command("git", &["init"]).await?;
    }
    Ok(())
}
