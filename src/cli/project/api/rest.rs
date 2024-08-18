use crate::cli::project::options::ProjectOptions;
use crate::utils::command;
use crate::utils::errors::ProjectError;
use crate::utils::modify_setting::{modify_settings, SettingsModification};
use std::fs;
use std::path::Path;

pub async fn create_rest_structure(options: &ProjectOptions) -> Result<(), ProjectError> {
    // Étape 1: Installer Django Rest Framework
    command::install_packages(&["djangorestframework"]).await?;

    // Étape 2: Ajouter 'rest_framework' à INSTALLED_APPS dans settings.py
    add_rest_framework_to_installed_apps(&options.name)?;

    println!("add_rest_framework_to_installed_apps ok");

    // Étape 3: Configurer les permissions et l'authentification dans settings.py
    configure_rest_framework_settings(&options.name)?;

    println!("configure_rest_framework_settings ok");

    // Étape 4: Création de la structure de l'API
    create_api_structure().await?;

    println!("create_api_structure ok");

    // Étape 5: Ajouter les routes d'API au fichier urls.py principal du projet
    add_api_urls_to_project(&options.name)?;

    println!("add_api_urls_to_project ok");

    Ok(())
}

fn add_rest_framework_to_installed_apps(project_name: &str) -> Result<(), ProjectError> {
    let modifications = vec![SettingsModification::AddToInstalledApps(
        "rest_framework".to_string(),
    )];

    modify_settings(project_name, &modifications)
}

fn configure_rest_framework_settings(project_name: &str) -> Result<(), ProjectError> {
    let modifications = vec![
        SettingsModification::AddOrUpdateDict(
            "REST_FRAMEWORK".to_string(),
            vec![
                ("DEFAULT_PERMISSION_CLASSES".to_string(), "['rest_framework.permissions.IsAuthenticated']".to_string()),
                ("DEFAULT_AUTHENTICATION_CLASSES".to_string(), "['rest_framework.authentication.SessionAuthentication', 'rest_framework.authentication.BasicAuthentication']".to_string()),
            ]
        ),
    ];

    // Ajout de logs pour vérifier si la modification a été tentée
    println!("Tentative de modification du fichier settings.py pour ajouter REST_FRAMEWORK.");

    modify_settings(project_name, &modifications)?;

    println!("Modification réussie de settings.py pour REST_FRAMEWORK.");

    Ok(())
}

// Étape 4: Création de la structure de l'API
pub async fn create_api_structure() -> Result<(), ProjectError> {
    // Étape 1: Vérifier si l'application 'api' existe déjà
    if !app_exists("api")? {
        // Étape 2: Créer l'application 'api' en utilisant la commande startapp
        create_django_app("api").await?;

        // Étape 3: Ajouter des fichiers spécifiques à l'API
        add_api_files("api")?;
        
        println!("L'application 'api' et sa structure ont été créées avec succès.");
    } else {
        println!("L'application 'api' existe déjà, saut de la création.");
    }

    Ok(())
}

fn app_exists(app_name: &str) -> Result<bool, ProjectError> {
    Ok(Path::new(app_name).exists())
}

async fn create_django_app(app_name: &str) -> Result<(), ProjectError> {
    // Exécuter la commande
    let output = command::run_command("django-admin", &["startapp", app_name])
        .await
        .map_err(|e| ProjectError::CommandFailed(e.to_string()))?;

    // Si la commande a réussi, retourner Ok(())
    if output.status.success() {
        Ok(())
    } else {
        Err(ProjectError::CommandFailed(format!(
            "Failed to create '{}' app. Command output: {}",
            app_name,
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn add_api_files(app_name: &str) -> Result<(), ProjectError> {
    let app_dir = Path::new(app_name);

    fs::write(
        app_dir.join("serializers.py"),
        "# Add your API serializers here",
    )
    .map_err(ProjectError::Io)?;
    fs::write(
        app_dir.join("urls.py"),
        "from django.urls import path, include\nfrom rest_framework.routers import DefaultRouter\n\n# Define your viewsets here and register them with the router\n\nrouter = DefaultRouter()\n# router.register(r'example', ExampleViewSet)\n\nurlpatterns = [\n    path('', include(router.urls)),\n]",
    ).map_err(ProjectError::Io)?;

    println!(
        "Les fichiers spécifiques à l'API ont été ajoutés dans l'application '{}'.",
        app_name
    );

    Ok(())
}

// Étape 5: Ajouter les routes d'API au fichier urls.py principal du projet
fn add_api_urls_to_project(project_name: &str) -> Result<(), ProjectError> {
    let modifications = vec![
        SettingsModification::AddOrUpdateVariable(
            "from django.urls import path, include".to_string(),
            "from django.urls import path, include".to_string(),
        ),
        SettingsModification::AddToInstalledApps("path('api/', include('api.urls')),".to_string()),
    ];

    modify_settings(project_name, &modifications)?;

    println!("Ajout des URLs API au fichier urls.py du projet.");

    Ok(())
}
