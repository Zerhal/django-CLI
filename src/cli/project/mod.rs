mod options;
mod setup;
mod structure;
mod files;
mod config;
pub mod api;

use crate::utils::errors::ProjectError;
use options::gather_project_options;
use setup::{check_prerequisites, initialize_version_control};
use structure::create_project_structure;
use files::generate_project_files;

pub async fn create_project() -> Result<(), ProjectError> {
    println!("Creating a new Django project...");

    check_prerequisites().await?;
    let options = gather_project_options().await?;
    
    create_project_structure(&options).await?;

    generate_project_files(&options).await?;
    
    config::configure_settings(&options).await?;
    initialize_version_control(&options).await?;
    // configure_frontend(&options).await?;

    println!("Project '{}' has been created successfully!", options.name);
    Ok(())
}