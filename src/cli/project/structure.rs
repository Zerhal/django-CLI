use crate::utils::errors::ProjectError;
use crate::utils::command::run_command;
use crate::cli::project::options::{ProjectOptions, ProjectType};
use std::fs;

pub async fn create_project_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    println!("Creating project structure...");

    create_django_project(&options.name).await?;
    std::env::set_current_dir(&options.name)?;

    create_apps_structure().await?;
    create_tests_structure().await?;
    create_config_structure(options).await?;
    create_static_structure().await?;
    create_templates_structure().await?;
    create_project_type_structure(options).await?;

    println!("Project structure created successfully.");
    Ok(())
}

async fn create_django_project(project_name: &str) -> Result<(), ProjectError> {
    run_command("django-admin", &["startproject", project_name]).await?;
    Ok(())
}

async fn create_apps_structure() -> Result<(), ProjectError> {
    fs::create_dir("apps")?;
    for app in &["core", "users"] {
        run_command("python", &["manage.py", "startapp", app, &format!("apps/{}", app)]).await?;
    }
    Ok(())
}

async fn create_tests_structure() -> Result<(), ProjectError> {
    fs::create_dir_all("tests/unit")?;
    fs::create_dir("tests/integration")?;
    fs::write("tests/__init__.py", "")?;
    Ok(())
}

async fn create_config_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}

async fn create_static_structure() -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}

async fn create_templates_structure() -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}

async fn create_project_type_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    match options.project_type {
        ProjectType::Rest => create_rest_structure().await,
        ProjectType::GraphQL => create_graphql_structure().await,
        ProjectType::Fullstack => create_fullstack_structure(options).await,
    }
}

async fn create_rest_structure() -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}

async fn create_graphql_structure() -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}

async fn create_fullstack_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    // Implementation similar to the original
    // ...

    Ok(())
}