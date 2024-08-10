use crate::config::ProjectConfig;
use std::fs;
use std::process::Command as ShellCommand;

pub fn create_django_project(project_name: &str) {
    println!("Création du projet Django...");

    ShellCommand::new("django-admin")
        .arg("startproject")
        .arg(project_name)
        .status()
        .expect("Failed to create Django project");

    println!("Projet Django {} créé avec succès.", project_name);
}

pub fn configure_django_project(config: &ProjectConfig) {
    println!("Configuration du projet...");

    // Créer les applications Django nécessaires en fonction de la configuration
    create_required_apps(config);

    // Configurer le fichier settings.py
    configure_settings_py(config);

    // Configurer le frontend s'il est requis
    configure_frontend(config);

    // Générer le fichier requirements.txt dynamiquement
    generate_requirements_txt(config);

    println!("Configuration du projet terminée.");
}

fn create_required_apps(config: &ProjectConfig) {
    if config.auth_enabled == "Oui" {
        create_django_app(&config.project_name, "authentication");
    }

    if config.api_type != "Aucune" {
        create_django_app(&config.project_name, "api");
    }
}

fn configure_frontend(config: &ProjectConfig) {
    if config.frontend_type == "React" {
        setup_react_frontend(&config.project_name);
    } else if config.frontend_type == "Django Template" {
        setup_django_template(&config.project_name);
    }
}

pub fn install_dependencies(project_name: &str) {
    println!("Installation des dépendances...");

    let project_dir = project_name.to_string();

    ShellCommand::new("pip")
        .arg("install")
        .arg("-r")
        .arg("requirements.txt")
        .current_dir(&project_dir)
        .status()
        .expect("Failed to install dependencies");

    println!("Dépendances installées avec succès.");
}

fn create_django_app(project_name: &str, app_name: &str) {
    println!("Création de l'application Django {}...", app_name);

    let project_dir = project_name.to_string();
    ShellCommand::new("python")
        .arg("manage.py")
        .arg("startapp")
        .arg(app_name)
        .current_dir(&project_dir)
        .status()
        .expect("Failed to create Django app");

    println!("Application Django {} créée avec succès.", app_name);
}

fn configure_settings_py(config: &ProjectConfig) {
    let settings_path = format!(
        "{}/{}/settings.py",
        config.project_name, config.project_name
    );
    let mut settings = fs::read_to_string(&settings_path).expect("Failed to read settings.py");

    if config.db_type != "SQLite" {
        settings.push_str("\n# Database configuration\nDATABASES = {...}");
    }

    if config.auth_enabled == "Oui" {
        settings.push_str("\n# Authentication settings\nINSTALLED_APPS += ['authentication']");
    }

    if config.email_service == "Oui" {
        settings.push_str("\n# Email service configuration\nEMAIL_BACKEND = 'django.core.mail.backends.smtp.EmailBackend'");
    }

    if config.cache_system == "Oui" {
        settings.push_str("\n# Cache settings\nCACHES = {...}");
    }

    fs::write(&settings_path, settings).expect("Failed to write settings.py");
}

fn setup_react_frontend(project_name: &str) {
    println!(
        "Configuration du frontend React pour le projet {}...",
        project_name
    );

    let frontend_dir = format!("{}/frontend", project_name);

    #[cfg(windows)]
    pub const NPM: &str = "npm.cmd";

    #[cfg(not(windows))]
    pub const NPM: &str = "npm";

    // Étape 1 : Initialiser le projet React avec create-react-app
    let status = ShellCommand::new(NPM)
        .arg("init")
        .arg("react-app")
        .arg(&frontend_dir)
        .status()
        .expect("Failed to create React app");

    if !status.success() {
        panic!("Failed to initialize React frontend");
    }

    // Étape 2 : Installer django-cors-headers pour gérer les CORS
    let project_dir = project_name.to_string();
    ShellCommand::new("pip")
        .arg("install")
        .arg("django-cors-headers")
        .current_dir(&project_dir)
        .status()
        .expect("Failed to install django-cors-headers");

    // Étape 3 : Configurer django-cors-headers dans settings.py
    let settings_path = format!("{}/{}/settings.py", project_name, project_name);
    let mut settings = fs::read_to_string(&settings_path).expect("Failed to read settings.py");

    settings.push_str("\n# CORS configuration\n");
    settings.push_str("INSTALLED_APPS += ['corsheaders']\n");
    settings.push_str("MIDDLEWARE = ['corsheaders.middleware.CorsMiddleware'] + MIDDLEWARE\n");
    settings.push_str("CORS_ORIGIN_ALLOW_ALL = True\n");

    // Configurer les fichiers statiques
    settings.push_str("\n# Static files (CSS, JavaScript, Images)\n");
    settings.push_str("STATICFILES_DIRS = [os.path.join(BASE_DIR, 'frontend/build/static')]\n");

    fs::write(&settings_path, settings).expect("Failed to write settings.py");

    println!(
        "Frontend React configuré avec succès pour le projet {}.",
        project_name
    );
}

fn setup_django_template(project_name: &str) {
    println!(
        "Configuration des templates Django pour le projet {}...",
        project_name
    );

    let templates_dir = format!("{}/templates", project_name);
    fs::create_dir_all(&templates_dir).expect("Failed to create templates directory");

    // Étape 1 : Mettre à jour settings.py pour inclure le dossier templates
    let settings_path = format!("{}/{}/settings.py", project_name, project_name);
    let mut settings = fs::read_to_string(&settings_path).expect("Failed to read settings.py");

    settings.push_str("\n# Template configuration\n");
    settings.push_str("TEMPLATES[0]['DIRS'] = [os.path.join(BASE_DIR, 'templates')]\n");

    fs::write(&settings_path, settings).expect("Failed to write settings.py");

    // Étape 2 : Créer un fichier base.html dans le dossier templates
    let base_html_path = format!("{}/base.html", templates_dir);
    let base_html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}My Django Project{% endblock %}</title>
    <link rel="stylesheet" href="{% static 'styles.css' %}">
</head>
<body>
    <header>
        <h1>Welcome to My Django Project</h1>
    </header>
    <main>
        {% block content %}
        <p>Hello, world!</p>
        {% endblock %}
    </main>
    <footer>
        <p>&copy; 2023 My Django Project</p>
    </footer>
</body>
</html>
"#;
    fs::write(base_html_path, base_html_content).expect("Failed to create base.html");

    println!(
        "Templates Django configurés avec succès pour le projet {}.",
        project_name
    );
}

fn generate_requirements_txt(config: &ProjectConfig) {
    println!("Génération du fichier requirements.txt...");

    let mut requirements = String::new();

    // Ajouter Django par défaut
    requirements.push_str("Django>=4.0,<5.0\n");

    // Ajouter djangorestframework si une API est requise
    if config.api_type == "REST" {
        requirements.push_str("djangorestframework>=3.13.0,<4.0\n");
    }

    // Ajouter graphene-django si GraphQL est requis
    if config.api_type == "GraphQL" {
        requirements.push_str("graphene-django>=2.15.0,<3.0\n");
    }

    // Ajouter des dépendances pour React si nécessaire
    if config.frontend_type == "React" {
        // Par exemple, ajouter django-cors-headers pour gérer les requêtes CORS avec React
        requirements.push_str("django-cors-headers>=3.5.0,<4.0\n");
    }

    // Écrire le fichier requirements.txt
    let requirements_path = format!("{}/requirements.txt", config.project_name);
    fs::write(requirements_path, requirements).expect("Failed to create requirements.txt");

    println!("Fichier requirements.txt généré avec succès.");
}
