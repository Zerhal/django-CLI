use std::io::{self, Write};
use crate::commands::setup_command;

/// Displays a prompt with a list of options and lets the user select one.
/// 
/// # Arguments
///
/// * `prompt` - A string slice that holds the prompt message.
/// * `options` - A slice of string slices representing the options to choose from.
///
/// # Returns
///
/// * A `String` containing the selected option.
///
/// # Example
///
/// ```
/// let choice = select_option("Please choose an option", &["Option 1", "Option 2", "Option 3"]);
/// println!("You selected: {}", choice);
/// ```
pub fn select_option(prompt: &str, options: &[&str]) -> String {
    println!("{}:", prompt);
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    print!("Please make a choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice = input.trim().parse::<usize>().unwrap_or(0);

    if choice > 0 && choice <= options.len() {
        options[choice - 1].to_string()
    } else {
        println!("Invalid choice, please try again.");
        select_option(prompt, options)
    }
}

/// Prompts the user to enter a name for the Django project.
///
/// # Returns
///
/// * A `String` containing the project name entered by the user.
///
/// # Example
///
/// ```
/// let project_name = get_project_name();
/// println!("Your Django project name is: {}", project_name);
/// ```
pub fn get_project_name() -> String {
    print!("Enter your Django project name: ");
    io::stdout().flush().unwrap();

    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    project_name.trim().to_string()
}

/// Sanitizes the project name by replacing invalid characters with underscores.
/// Ensures the project name is a valid Python identifier.
///
/// # Arguments
///
/// * `name` - A string slice representing the project name to sanitize.
///
/// # Returns
///
/// * A `String` containing the sanitized project name.
///
/// # Example
///
/// ```
/// let sanitized_name = sanitize_project_name("My Project!");
/// println!("Sanitized project name: {}", sanitized_name);
/// ```
pub fn sanitize_project_name(name: &str) -> String {
    let sanitized = name
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
    if sanitized.chars().next().unwrap().is_ascii_digit() {
        format!("_{}", sanitized)
    } else {
        sanitized
    }
}

/// Prints the metadata for the CLI application, including the name, version, author, and description.
pub fn print_metadata() {
    let cmd = setup_command();
    println!("{}", cmd.get_name());

    println!("----------------------------------------");
    println!("     Welcome to the Django CLI Generator     ");
    println!("----------------------------------------");

    println!(
        "Version: {}",
        cmd.get_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Author: {}",
        cmd.get_author()
            .map(|a| a.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "About: {}",
        cmd.get_about()
            .map(|a| a.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(); // Adds a blank line for readability
}
