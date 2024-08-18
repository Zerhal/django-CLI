use crate::utils::errors::ProjectError;
use std::process::Output;
use tokio::process::Command;

pub async fn run_command(command: &str, args: &[&str]) -> Result<Output, ProjectError> {
    let output = Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(ProjectError::CommandFailed(format!(
            "Command failed: {} {}\nstdout: {}\nstderr: {}",
            command,
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(output)
}

pub async fn install_packages(packages: &[&str]) -> Result<(), ProjectError> {
    let pip_command = if Command::new("pip").arg("--version").output().await.is_ok() {
        "pip"
    } else if Command::new("pip3").arg("--version").output().await.is_ok() {
        "pip3"
    } else {
        return Err(ProjectError::CommandFailed(
            "Ni 'pip' ni 'pip3' n'ont été trouvés. Veuillez installer Python et pip.".into(),
        ));
    };

    let status = Command::new(pip_command)
        .arg("install")
        .args(packages)
        .status()
        .await
        .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

    if !status.success() {
        return Err(ProjectError::CommandFailed(format!(
            "Échec de l'installation des packages : {}",
            packages.join(", ")
        )));
    }

    println!(
        "Les packages suivants ont été installés avec succès : {}",
        packages.join(", ")
    );

    Ok(())
}