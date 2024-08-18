use crate::cli::project::options::{DatabaseType, ProjectOptions, ProjectType};
use crate::utils::command::run_command;
use crate::utils::errors::ProjectError;
use anyhow::Result;
use std::fs;
use std::path::Path;

use tera::{Context, Tera};

use super::config::configure_frontend;

pub async fn create_project_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    println!("Creating project structure...");

    create_django_project(&options.name).await?;
    std::env::set_current_dir(&options.name)?;
    create_tests_structure().await?;
    create_config_structure(options).await?;
    create_static_structure().await?;

    // TODO : create project type structure
    create_project_type_structure(options).await?;

    println!("Project structure created successfully.");
    Ok(())
}

async fn create_django_project(project_name: &str) -> Result<(), ProjectError> {
    // Vérifier si le répertoire du projet existe déjà
    if Path::new(project_name).exists() {
        return Err(ProjectError::CommandFailed(format!(
            "Le répertoire '{}' existe déjà. Veuillez choisir un autre nom de projet.",
            project_name
        )));
    }

    // Tenter de créer le projet Django
    match run_command("django-admin", &["startproject", project_name]).await {
        Ok(_) => {
            println!(
                "Le projet Django '{}' a été créé avec succès.",
                project_name
            );
            Ok(())
        }
        Err(e) => Err(ProjectError::CommandFailed(format!(
            "Échec de la création du projet Django '{}': {}",
            project_name, e
        ))),
    }
}

async fn create_tests_structure() -> Result<(), ProjectError> {
    fs::create_dir_all("tests/unit")?;
    fs::create_dir("tests/integration")?;
    fs::write("tests/__init__.py", "")?;
    Ok(())
}

async fn create_config_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    // Inclure le template Tera embarqué dans le binaire
    let template_content = include_str!("../../../templates/setup/env_template.tpl");
    let mut tera = Tera::default();
    tera.add_raw_template("env_template", template_content)
        .map_err(ProjectError::TemplateError)?;

    let env_files = vec!["dev.env", "prod.env", "test.env"];
    let db_engine = match options.database.db_type {
        DatabaseType::Postgres => "django.db.backends.postgresql",
        DatabaseType::MySQL => "django.db.backends.mysql",
        DatabaseType::SQLite => "django.db.backends.sqlite3",
    };

    // Contexte pour le fichier dev.env
    let mut context_dev = Context::new();
    context_dev.insert("db_engine", db_engine);
    context_dev.insert("db_name", &options.database.name);
    context_dev.insert("db_user", &options.database.user);
    context_dev.insert("db_password", &options.database.password);
    context_dev.insert("db_host", &options.database.host);
    context_dev.insert("db_port", &options.database.port);

    // Contexte par défaut pour les autres fichiers (prod.env, test.env)
    let mut context_default = Context::new();
    context_default.insert("db_engine", db_engine);
    context_default.insert("db_name", "database_name");
    context_default.insert("db_user", "database_user");
    context_default.insert("db_password", "database_password");
    context_default.insert("db_host", "database_host");
    context_default.insert("db_port", "database_port");

    for file in env_files {
        let rendered = if file == "dev.env" {
            tera.render("env_template", &context_dev)
        } else {
            tera.render("env_template", &context_default)
        }
        .map_err(ProjectError::TemplateError)?;

        fs::write(file, rendered).map_err(|e| {
            ProjectError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Impossible d'écrire dans le fichier '{}': {}", file, e),
            ))
        })?;
    }

    Ok(())
}
async fn create_static_structure() -> Result<(), ProjectError> {
    fs::create_dir_all("static/css")?;
    fs::create_dir("static/js")?;
    fs::create_dir("static/images")?;
    Ok(())
}

async fn create_project_type_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    match options.project_type {
        ProjectType::Rest => crate::cli::project::api::rest::create_rest_structure(options).await,
        ProjectType::GraphQL => create_graphql_structure().await,
        ProjectType::Fullstack => configure_frontend(options).await,
    }
}



async fn create_graphql_structure() -> Result<(), ProjectError> {
    fs::create_dir("graphql")?;
    fs::write("graphql/__init__.py", "")?;
    fs::write("graphql/schema.py", "import graphene\n\nclass Query(graphene.ObjectType):\n    pass\n\nschema = graphene.Schema(query=Query)")?;
    Ok(())
}

// fn generate_project_files(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
//     println!("Generating project files...");
//     let mut tera = Tera::default();
//     tera.add_raw_templates(vec![
//         (
//             ".gitignore",
//             include_str!("../../../templates/setup/.gitignore.tpl"),
//         ),
//         (
//             "Dockerfile",
//             include_str!("../../../templates/setup/Dockerfile.tpl"),
//         ),
//         (
//             "README.md",
//             include_str!("../../../templates/setup/README.md.tpl"),
//         ),
//         (
//             "project_structure.yml",
//             include_str!("../../../templates/project/project_structure.tpl"),
//         ),
//     ])?;

//     let mut context = tera::Context::new();
//     context.insert("project_name", &options.name);
//     context.insert("project_type", &options.project_type);
//     context.insert("db_type", &options.db_type);
//     context.insert("auth_system", &options.auth_system);

//     // TODO: Generate files using Tera
//     Ok(())
// }
