use std::io::{self, Write};

use crate::commands::setup_command;

pub fn select_option(prompt: &str, options: &[&str]) -> String {
    println!("{}:", prompt);
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    print!("Veuillez faire un choix: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice = input.trim().parse::<usize>().unwrap_or(0);

    if choice > 0 && choice <= options.len() {
        options[choice - 1].to_string()
    } else {
        println!("Choix invalide, veuillez réessayer.");
        select_option(prompt, options)
    }
}

pub fn get_project_name() -> String {
    print!("Entrez le nom de votre projet Django : ");
    io::stdout().flush().unwrap();

    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    project_name.trim().to_string()
}

pub fn sanitize_project_name(name: &str) -> String {
    let sanitized = name.trim().replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
    if sanitized.chars().next().unwrap().is_ascii_digit() {
        format!("_{}", sanitized)
    } else {
        sanitized
    }
}

pub fn print_metadata() {
    let cmd = setup_command();
    println!("{}", cmd.get_name());

    println!("----------------------------------------");
    println!("    Bienvenue dans le générateur CLI    ");
    println!("            pour Django                 ");
    println!("----------------------------------------");

    println!(
        "Version: {}",
        cmd.get_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Auteur: {}",
        cmd.get_author()
            .map(|a| a.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "À propos: {}",
        cmd.get_about()
            .map(|a| a.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(); // Ajouter une ligne vide pour la lisibilité
}
