use crate::utils::check::{get_python_version, is_command_available};
use std::env;
use std::process::Command as ShellCommand;

const PYTHON_INSTALL_URL_WINDOWS: &str =
    "https://www.python.org/ftp/python/3.12.0/python-3.12.0-amd64.exe";
const DJANGO_PYTHON_COMPATIBILITY: &[(u32, u32, &str)] = &[
    (3, 6, "3.2"),
    (3, 7, "3.2"),
    (3, 8, "4.2.8"),
    (3, 9, "4.2.8"),
    (3, 10, "4.2.8"),
    (3, 11, "4.2.8"),
    (3, 12, "4.2.8"),
];

/// Ensures that Python is installed and compatible with Django.
/// If no compatible Python version is found, installs Python 3.12 and sets Django 4.2.8 as the target version.
///
/// # Returns
///
/// * A `String` containing the path or command for the installed Python interpreter.
///
/// # Panics
///
/// * If the Python installation fails or no compatible Python version is found.
///
/// # Example
///
/// ```
/// let python_command = install_python();
/// println!("Using Python command: {}", python_command);
/// ```
pub fn install_python() -> String {
    let python_commands = ["python", "python3"];
    let mut compatible_command = None;
    let mut _django_version = None;

    for command in &python_commands {
        if is_command_available(command) {
            if let Some((major, minor)) = get_python_version(command) {
                for &(py_major, py_minor, django_ver) in DJANGO_PYTHON_COMPATIBILITY {
                    if major == py_major && minor == py_minor {
                        compatible_command = Some(command.to_string());
                        _django_version = Some(django_ver);
                        break;
                    }
                }
            }
        }
    }

    if compatible_command.is_none() {
        println!("No compatible Python version found. Installing Python 3.12...");

        match env::consts::OS {
            "windows" => {
                if let Err(err) = ShellCommand::new("cmd")
                    .args(&["/C", "start", PYTHON_INSTALL_URL_WINDOWS])
                    .status()
                {
                    panic!("Failed to download Python for Windows: {}", err);
                }
                println!("Please follow the instructions to install Python.");
            }
            "macos" => {
                if let Err(err) = ShellCommand::new("brew")
                    .arg("install")
                    .arg("python@3.12")
                    .status()
                {
                    panic!("Failed to install Python via Homebrew: {}", err);
                }
            }
            "linux" => {
                if let Err(err) = ShellCommand::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg("python3.12")
                    .status()
                {
                    panic!("Failed to install Python via apt: {}", err);
                }
            }
            _ => panic!("Unsupported operating system"),
        }

        compatible_command = Some("python3.12".to_string());
        _django_version = Some("4.2.8");
    }

    compatible_command.expect("Python installation failed or is still unavailable.")
}

/// Ensures that pip is installed. If not, installs it using the Python interpreter found or installed by `install_python`.
///
/// # Returns
///
/// * A `String` containing the command to run pip.
///
/// # Panics
///
/// * If pip installation fails or pip is still unavailable after installation.
///
/// # Example
///
/// ```
/// let pip_command = install_pip();
/// println!("Using pip command: {}", pip_command);
/// ```
pub fn install_pip() -> String {
    let python_command = install_python(); // Ensure Python is installed and get the command

    let pip_commands = ["pip", "pip3"];

    for command in &pip_commands {
        if is_command_available(command) {
            return command.to_string();
        }
    }

    println!("pip is not installed. Installing pip...");

    if let Err(err) = ShellCommand::new(&python_command)
        .arg("-m")
        .arg("ensurepip")
        .status()
    {
        panic!("Failed to install pip via ensurepip: {}", err);
    }

    for command in &pip_commands {
        if is_command_available(command) {
            return command.to_string();
        }
    }

    panic!("Pip installation failed or is still unavailable.");
}

/// Ensures that Django is installed using pip. If not installed, it installs Django 4.2.8,
/// particularly if Python 3.12 was installed by `install_python`.
///
/// # Panics
///
/// * If Django installation fails or `django-admin` is still unavailable after installation.
///
/// # Example
///
/// ```
/// install_django();
/// println!("Django installed and ready to use.");
/// ```
pub fn install_django() {
    let pip_command = install_pip(); // Ensure pip is installed and get the command

    let django_version = "4.2.8"; // Force the use of Django 4.2.8 in case of Python 3.12

    if is_command_available("django-admin") {
        println!("Django-admin is already installed.");
    } else {
        println!("Django-admin is not installed. Installing Django...");

        if let Err(err) = ShellCommand::new(&pip_command)
            .arg("install")
            .arg(format!("django=={}", django_version))
            .status()
        {
            panic!("Failed to run pip to install Django: {}", err);
        }

        if !is_command_available("django-admin") {
            panic!("Django installation failed.");
        } else {
            println!("Django {} was successfully installed.", django_version);
        }
    }
}
