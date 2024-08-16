use crate::utils::errors::ProjectError;
use tokio::process::Command;

pub async fn run_command(command: &str, args: &[&str]) -> Result<(), ProjectError> {
    let output = Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(ProjectError::CommandFailed(format!(
            "Command failed: {} {}",
            command,
            args.join(" ")
        ))
        .into());
    }
    Ok(())
}