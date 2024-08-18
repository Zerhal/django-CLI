// src/main.rs

mod cli;
mod utils;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(
    name = "Django CLI",
    version = "1.0",
    author = "Zerhal <jessy.viotti90@gmail.com>",
    about = "Django CLI",
    long_about = Some("Ultimate CLI for fast and easy Django development."),
    after_help = "More information at https://github.com/Zerhal/django-CLI", 
    styles = styles()
)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[command(
        about = "Create a new Django project",
        long_about = Some("This command creates a new Django project with the given name."),
        alias = "new",
        after_help = "Example:\n  django-cli create-project"
    )]
    CreateProject,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::CreateProject => {
            if let Err(e) = cli::project::create_project().await {
                eprintln!("{}", format!("Error: {}", e));
            } else {
                println!("{}", "Project created successfully!");
            }
        },
    }
}