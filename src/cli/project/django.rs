// src/cli/django.rs

use std::fs;
use std::process::Command;

pub fn create_django_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initialisation du projet Django avec django-admin...");
    Command::new("django-admin")
        .args(&["startproject", name])
        .output()?;
    println!("Projet Django '{}' initialisé.", name);
    Ok(())
}

pub fn configure_settings(name: &str, database: &str, auth: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration de settings.py...");

    let settings_path = format!("{}/{}/settings.py", name, name);
    let mut settings_content = fs::read_to_string(&settings_path)?;

    let additional_settings = format!(
        r#"
        DATABASES = {{
            'default': {{
                'ENGINE': 'django.db.backends.{}',
                'NAME': '{}_db',
            }}
        }}

        AUTHENTICATION_BACKENDS = [
            'django.contrib.auth.backends.ModelBackend',
            // Ajoutez d'autres backends d'authentification ici si nécessaire
        ]

        // Configuration supplémentaire basée sur le type d'authentification
        {}
        "#,
        database,
        name,
        match auth {
            "jwt" => "REST_FRAMEWORK = { 'DEFAULT_AUTHENTICATION_CLASSES': [ 'rest_framework_simplejwt.authentication.JWTAuthentication', ], }",
            "oauth" => "INSTALLED_APPS += ['oauth2_provider']",
            _ => ""
        }
    );

    settings_content.push_str(&additional_settings);
    fs::write(settings_path, settings_content)?;

    println!("settings.py configuré.");
    Ok(())
}
