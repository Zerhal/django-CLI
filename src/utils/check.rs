use std::process::{Command as ShellCommand, Stdio};

/// Checks if a given command is available on the system by trying to run it with the `--version` argument.
///
/// # Arguments
///
/// * `command` - A string slice representing the command to check (e.g., "python").
///
/// # Returns
///
/// * `true` if the command is available (i.e., it runs successfully with the `--version` argument),
/// * `false` otherwise.
///
/// # Example
///
/// ```
/// let available = is_command_available("python");
/// if available {
///     println!("Python is installed.");
/// } else {
///     println!("Python is not installed.");
/// }
/// ```
pub fn is_command_available(command: &str) -> bool {
    ShellCommand::new(command)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// Retrieves the version of Python by running the given command with the `--version` argument.
///
/// # Arguments
///
/// * `command` - A string slice representing the Python command to check (e.g., "python" or "python3").
///
/// # Returns
///
/// * `Some((major, minor))` where `major` and `minor` are the major and minor version numbers of Python,
/// * `None` if the version could not be determined.
///
/// # Example
///
/// ```
/// if let Some((major, minor)) = get_python_version("python") {
///     println!("Python version: {}.{}", major, minor);
/// } else {
///     println!("Failed to retrieve Python version.");
/// }
/// ```
pub fn get_python_version(command: &str) -> Option<(u32, u32)> {
    let output = ShellCommand::new(command).arg("--version").output().ok()?;

    let version_string = String::from_utf8_lossy(&output.stdout);
    let version_parts: Vec<&str> = version_string
        .split_whitespace()
        .nth(1)?
        .split('.')
        .collect();

    if version_parts.len() >= 2 {
        let major = version_parts[0].parse::<u32>().ok()?;
        let minor = version_parts[1].parse::<u32>().ok()?;
        Some((major, minor))
    } else {
        None
    }
}
