use crate::cli::project::options::{AuthSystem, DatabaseOptions, ProjectOptions};
use crate::utils::errors::ProjectError;
use crate::utils::modify_setting::{modify_settings, SettingsModification};

pub async fn configure_settings(options: &ProjectOptions) -> Result<(), ProjectError> {
    println!("Configuring settings.py...");

    // Créer une liste de modifications à apporter à settings.py
    let mut modifications = Vec::new();

    // Ajouter la configuration de la base de données
    let db_config = generate_db_config(&options.database);
    modifications.push(SettingsModification::AddOrUpdateDict(
        "DATABASES".to_string(),
        db_config,
    ));

    // Ajouter la configuration du système d'authentification
    let auth_config = generate_auth_config(&options.auth_system);
    modifications.push(SettingsModification::AddOrUpdateDict(
        "REST_FRAMEWORK".to_string(),
        auth_config,
    ));

    // Appliquer les modifications à settings.py
    modify_settings(&options.name, &modifications)?;

    Ok(())
}

// Génère la configuration de la base de données sous forme de tuples (clé, valeur)
fn generate_db_config(db_options: &DatabaseOptions) -> Vec<(String, String)> {
    vec![
        (
            "ENGINE".to_string(),
            db_options.db_type.to_engine_string().to_string(),
        ),
        ("NAME".to_string(), db_options.name.clone()),
        ("USER".to_string(), db_options.user.clone()),
        ("PASSWORD".to_string(), db_options.password.clone()),
        ("HOST".to_string(), db_options.host.clone()),
        ("PORT".to_string(), db_options.port.clone()),
    ]
}

// Génère la configuration du système d'authentification sous forme de tuples (clé, valeur)
fn generate_auth_config(auth_system: &AuthSystem) -> Vec<(String, String)> {
    match auth_system {
        AuthSystem::Django => Vec::new(),  // Pas de configuration spécifique
        AuthSystem::JWT => vec![
            ("DEFAULT_AUTHENTICATION_CLASSES".to_string(), "['rest_framework_simplejwt.authentication.JWTAuthentication']".to_string()),
            ("AUTH_HEADER_TYPES".to_string(), "('JWT',)".to_string()),
        ],
        AuthSystem::OAuth => vec![
            ("INSTALLED_APPS".to_string(), "['oauth2_provider', 'rest_framework_social_oauth2']".to_string()),
            ("AUTHENTICATION_BACKENDS".to_string(), "('oauth2_provider.backends.OAuth2Backend', 'django.contrib.auth.backends.ModelBackend')".to_string()),
        ],
    }
}

pub async fn configure_frontend(options: &ProjectOptions) -> Result<(), ProjectError> {
    if let Some(framework) = &options.frontend_framework {
        println!("Configuring frontend with {}...", framework);
        match framework.as_str() {
            "react" => configure_react().await?,
            "vue" => configure_vue().await?,
            "angular" => configure_angular().await?,
            _ => return Err(ProjectError::InvalidFrontendFramework(framework.clone())),
        }
    }
    Ok(())
}

async fn configure_react() -> Result<(), ProjectError> {
    // Command::new("npx")
    //     .args(&["create-react-app", "frontend"])
    //     .status()?;
    Ok(())
}

async fn configure_vue() -> Result<(), ProjectError> {
    // Command::new("vue").args(&["create", "frontend"]).status()?;
    Ok(())
}

async fn configure_angular() -> Result<(), ProjectError> {
    // Command::new("ng").args(&["new", "frontend"]).status()?;
    Ok(())
}
