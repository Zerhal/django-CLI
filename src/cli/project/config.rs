use crate::utils::errors::ProjectError;
use crate::cli::project::options::{ProjectOptions, AuthSystem, DatabaseOptions};

pub async fn configure_settings(options: &ProjectOptions) -> Result<(), ProjectError> {
    println!("Configuring settings.py...");
    
    // Read the existing settings.py file
    let mut settings = std::fs::read_to_string("your_project_name/settings.py")?;

    // Update database configuration
    let db_config = generate_db_config(&options.database);
    settings = settings.replace("DATABASES = {}", &db_config);

    // Update authentication configuration
    let auth_config = generate_auth_config(&options.auth_system);
    settings.push_str(&auth_config);

    // Write the updated settings back to the file
    std::fs::write("your_project_name/settings.py", settings)?;

    Ok(())
}

fn generate_db_config(db_options: &DatabaseOptions) -> String {
    // Generate database configuration based on the selected database type
    format!(
        "DATABASES = {{
            'default': {{
                'ENGINE': '{}',
                'NAME': '{}',
                'USER': '{}',
                'PASSWORD': '{}',
                'HOST': '{}',
                'PORT': '{}',
            }}
        }}",
        db_options.db_type.to_engine_string(),
        db_options.name,
        db_options.user,
        db_options.password,
        db_options.host,
        db_options.port
    )
}

fn generate_auth_config(auth_system: &AuthSystem) -> String {
    match auth_system {
        AuthSystem::Django => "# Using default Django authentication".to_string(),
        AuthSystem::JWT => {
            "
REST_FRAMEWORK = {
    'DEFAULT_AUTHENTICATION_CLASSES': [
        'rest_framework_simplejwt.authentication.JWTAuthentication',
    ],
}

SIMPLE_JWT = {
    'AUTH_HEADER_TYPES': ('JWT',),
}
".to_string()
        },
        AuthSystem::OAuth => {
            "
INSTALLED_APPS += [
    'oauth2_provider',
    'rest_framework_social_oauth2',
]

AUTHENTICATION_BACKENDS = (
    'oauth2_provider.backends.OAuth2Backend',
    'django.contrib.auth.backends.ModelBackend',
)
".to_string()
        },
    }
}

pub async fn configure_frontend(options: &ProjectOptions) -> Result<(), ProjectError> {
    if let Some(framework) = &options.frontend_framework {
        println!("Configuring frontend with {}...", framework);
        match framework.as_str() {
            "react" => configure_react().await?,
            "vue" => configure_vue().await?,
            "angular" => configure_angular().await?,
            _ => return Err(ProjectError::UnsupportedFramework(framework.to_string())),
        }
    }
    Ok(())
}

async fn configure_react() -> Result<(), ProjectError> {
    // Configure React frontend
    // ...
    Ok(())
}

async fn configure_vue() -> Result<(), ProjectError> {
    // Configure Vue frontend
    // ...
    Ok(())
}

async fn configure_angular() -> Result<(), ProjectError> {
    // Configure Angular frontend
    // ...
    Ok(())
}
