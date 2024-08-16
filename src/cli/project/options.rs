use crate::utils::errors::ProjectError;
use dialoguer::{Input, MultiSelect, Select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectType {
    Rest,
    GraphQL,
    Fullstack,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    Postgres,
    MySQL,
    SQLite,
}

impl DatabaseType {
    pub fn to_engine_string(&self) -> &str {
        match self {
            DatabaseType::Postgres => "django.db.backends.postgresql",
            DatabaseType::MySQL => "django.db.backends.mysql",
            DatabaseType::SQLite => "django.db.backends.sqlite3",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthSystem {
    Django,
    JWT,
    OAuth,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManager {
    Venv,
    Poetry,
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AdditionalOption {
    Git,
    Docker,
    Readme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseOptions {
    pub db_type: DatabaseType,
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectOptions {
    pub name: String,
    pub project_type: ProjectType,
    pub frontend_framework: Option<String>,
    pub database: DatabaseOptions,
    pub auth_system: AuthSystem,
    pub package_manager: PackageManager,
    pub additional_options: Vec<AdditionalOption>,
}

pub async fn gather_project_options() -> Result<ProjectOptions, ProjectError> {
    let name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact_text()?;

    let project_type = Select::new()
        .with_prompt("Select the project type")
        .items(&["Rest", "GraphQL", "Fullstack"])
        .interact()
        .map(|i| match i {
            0 => ProjectType::Rest,
            1 => ProjectType::GraphQL,
            2 => ProjectType::Fullstack,
            _ => unreachable!(),
        })?;

    let frontend_framework = if matches!(project_type, ProjectType::Fullstack) {
        Some(
            Select::new()
                .with_prompt("Select the frontend framework")
                .items(&["React", "Vue", "Angular"])
                .interact()
                .map(|i| match i {
                    0 => "react",
                    1 => "vue",
                    2 => "angular",
                    _ => unreachable!(),
                })?
                .to_string(),
        )
    } else {
        None
    };

    let db_type = Select::new()
        .with_prompt("Select the database type")
        .items(&["SQLite", "Postgres", "MySQL"]) // Corrected variant name
        .interact()
        .map(|i| match i {
            0 => DatabaseType::SQLite,
            1 => DatabaseType::Postgres, // Corrected variant name
            2 => DatabaseType::MySQL,
            _ => unreachable!(),
        })?;

    let database = DatabaseOptions {
        db_type: db_type.clone(), // Clone the value here
        name: "your_db_name".to_string(),
        user: "your_db_user".to_string(),
        password: "your_db_password".to_string(),
        host: "localhost".to_string(),
        port: match db_type {
            DatabaseType::Postgres => "5432".to_string(),
            DatabaseType::MySQL => "3306".to_string(),
            _ => "".to_string(),
        },
    };

    let auth_system = Select::new()
        .with_prompt("Select the authentication system")
        .items(&["Django", "JWT", "OAuth"])
        .interact()
        .map(|i| match i {
            0 => AuthSystem::Django,
            1 => AuthSystem::JWT,
            2 => AuthSystem::OAuth,
            _ => unreachable!(),
        })?;

    let package_manager = Select::new()
        .with_prompt("Select the package manager")
        .items(&["Venv", "Poetry", "None"])
        .interact()
        .map(|i| match i {
            0 => PackageManager::Venv,
            1 => PackageManager::Poetry,
            2 => PackageManager::None,
            _ => unreachable!(),
        })?;

    let additional_options = MultiSelect::new()
        .with_prompt("Select additional options")
        .items(&["Git", "Docker", "Readme"])
        .interact()?
        .into_iter()
        .map(|i| match i {
            0 => AdditionalOption::Git,
            1 => AdditionalOption::Docker,
            2 => AdditionalOption::Readme,
            _ => unreachable!(),
        })
        .collect();

    Ok(ProjectOptions {
        name,
        project_type,
        frontend_framework,
        database,
        auth_system,
        package_manager,
        additional_options,
    })
}
