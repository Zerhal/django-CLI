use crate::utils::errors::ProjectError;
use dialoguer::{Input, MultiSelect, Select};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectType {
    Rest,
    GraphQL,
    Fullstack,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Rest => write!(f, "Rest"),
            ProjectType::GraphQL => write!(f, "GraphQL"),
            ProjectType::Fullstack => write!(f, "Fullstack"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    Postgres,
    MySQL,
    SQLite,
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "Postgres"),
            DatabaseType::MySQL => write!(f, "MySQL"),
            DatabaseType::SQLite => write!(f, "SQLite"),
        }
    }
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

impl fmt::Display for AuthSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthSystem::Django => write!(f, "Django"),
            AuthSystem::JWT => write!(f, "JWT"),
            AuthSystem::OAuth => write!(f, "OAuth"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManager {
    Venv,
    Poetry,
    None,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Venv => write!(f, "Venv"),
            PackageManager::Poetry => write!(f, "Poetry"),
            PackageManager::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AdditionalOption {
    Git,
    Docker,
    Readme,
}

impl fmt::Display for AdditionalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdditionalOption::Git => write!(f, "Git"),
            AdditionalOption::Docker => write!(f, "Docker"),
            AdditionalOption::Readme => write!(f, "Readme"),
        }
    }
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

    let project_type = match Select::new()
        .with_prompt("Select the project type")
        .items(&[
            ProjectType::Rest.to_string(),
            ProjectType::GraphQL.to_string(),
            ProjectType::Fullstack.to_string(),
        ])
        .interact()?
    {
        0 => ProjectType::Rest,
        1 => ProjectType::GraphQL,
        2 => ProjectType::Fullstack,
        _ => unreachable!(),
    };

    let frontend_framework = if let ProjectType::Fullstack = project_type {
        Some(
            Select::new()
                .with_prompt("Select the frontend framework")
                .items(&["React", "Vue", "Angular"])
                .interact()?
                .to_string(),
        )
    } else {
        None
    };

    let db_type = match Select::new()
        .with_prompt("Select the database type")
        .items(&[
            DatabaseType::SQLite.to_string(),
            DatabaseType::Postgres.to_string(),
            DatabaseType::MySQL.to_string(),
        ])
        .interact()?
    {
        0 => DatabaseType::SQLite,
        1 => DatabaseType::Postgres,
        2 => DatabaseType::MySQL,
        _ => unreachable!(),
    };

    let database = DatabaseOptions {
        db_type: db_type.clone(), // Cloner explicitement `db_type` ici
        name: Input::new()
            .with_prompt("Enter the database name")
            .default("db".to_string())
            .interact_text()?,
        user: Input::new()
            .with_prompt("Enter the database user")
            .default("user".to_string())
            .interact_text()?,
        password: Input::new()
            .with_prompt("Enter the database password")
            .default("password".to_string())
            .interact_text()?,
        host: Input::new()
            .with_prompt("Enter the database host")
            .default("localhost".to_string())
            .interact_text()?,
        port: match db_type {
            DatabaseType::Postgres => "5432".to_string(),
            DatabaseType::MySQL => "3306".to_string(),
            DatabaseType::SQLite => "N/A".to_string(),
        },
    };

    let auth_system = match Select::new()
        .with_prompt("Select the authentication system")
        .items(&[
            AuthSystem::Django.to_string(),
            AuthSystem::JWT.to_string(),
            AuthSystem::OAuth.to_string(),
        ])
        .interact()?
    {
        0 => AuthSystem::Django,
        1 => AuthSystem::JWT,
        2 => AuthSystem::OAuth,
        _ => unreachable!(),
    };

    let package_manager = match Select::new()
        .with_prompt("Select the package manager")
        .items(&[
            PackageManager::Venv.to_string(),
            PackageManager::Poetry.to_string(),
            PackageManager::None.to_string(),
        ])
        .interact()?
    {
        0 => PackageManager::Venv,
        1 => PackageManager::Poetry,
        2 => PackageManager::None,
        _ => unreachable!(),
    };

    let additional_options = MultiSelect::new()
        .with_prompt("Select additional options")
        .items(&[
            AdditionalOption::Git.to_string(),
            AdditionalOption::Docker.to_string(),
            AdditionalOption::Readme.to_string(),
        ])
        .interact()?
        .into_iter()
        .map(|i| match i {
            0 => AdditionalOption::Git,
            1 => AdditionalOption::Docker,
            2 => AdditionalOption::Readme,
            _ => unreachable!(),
        })
        .collect::<Vec<AdditionalOption>>();

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
