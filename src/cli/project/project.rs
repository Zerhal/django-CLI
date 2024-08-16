// src/cli/project/project.rs
use crate::utils::tools::get_compatible_django_version;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::error::Error;
use std::process::Command;
use tera::Tera;
use std::fs;

// Structure pour stocker les options du projet
struct ProjectOptions {
    name: String,
    project_type: String,
    frontend_framework: Option<String>,
    db_type: String,
    auth_system: String,
    package_manager: String,
    additional_options: Vec<String>,
}

pub fn create_project() -> Result<(), Box<dyn Error>> {
    println!("Creating a new Django project...");

    check_prerequisites()?;
    let options = gather_project_options()?;
    create_project_structure(&options)?;
    generate_project_files(&options)?;
    configure_settings(&options)?;
    initialize_version_control(&options)?;
    configure_frontend(&options)?;

    println!("Project '{}' has been created successfully!", options.name);
    Ok(())
}

fn check_prerequisites() -> Result<(), Box<dyn Error>> {
    let python_version = check_and_install_python()?;
    check_and_install_pip(&python_version)?;
    let django_version = get_compatible_django_version(&python_version)
        .ok_or("No compatible Django version found")?;

    println!("Detected Python version: {}", python_version);
    println!("Recommended Django version: {}", django_version);

    if !Confirm::new()
        .with_prompt("Do you want to continue with the project creation?")
        .interact()?
    {
        println!("Project creation canceled.");
        return Err("User canceled project creation".into());
    }

    Ok(())
}

fn gather_project_options() -> Result<ProjectOptions, Box<dyn Error>> {
    let name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact_text()?;

    let project_types = vec!["rest", "graphql", "fullstack"];
    let project_type = project_types[Select::new()
        .with_prompt("Select the project type")
        .items(&project_types)
        .interact()?];

    let frontend_framework = if project_type == "fullstack" {
        let frameworks = vec!["react", "vue", "angular"];
        Some(
            frameworks[Select::new()
                .with_prompt("Select the frontend framework")
                .items(&frameworks)
                .interact()?]
            .to_string(),
        )
    } else {
        None
    };

    let db_types = vec!["sqlite", "postgres", "mysql"];
    let db_type = db_types[Select::new()
        .with_prompt("Select the database type")
        .items(&db_types)
        .interact()?];

    let auth_systems = vec!["django", "jwt", "oauth"];
    let auth_system = auth_systems[Select::new()
        .with_prompt("Select the authentication system")
        .items(&auth_systems)
        .interact()?];

    let package_managers = vec!["Venv", "Poetry", "None"];
    let package_manager = package_managers[Select::new()
        .with_prompt("Select the package manager")
        .items(&package_managers)
        .interact()?];

    let options = vec!["Git", "Docker", "Readme"];
    let selections = MultiSelect::new()
        .with_prompt("Select additional options")
        .items(&options)
        .interact()?;

    let additional_options: Vec<String> =
        selections.iter().map(|&i| options[i].to_string()).collect();

    Ok(ProjectOptions {
        name,
        project_type: project_type.to_string(),
        frontend_framework,
        db_type: db_type.to_string(),
        auth_system: auth_system.to_string(),
        package_manager: package_manager.to_string(),
        additional_options,
    })
}

fn create_project_structure(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    println!("Creating project structure...");

    // Créer le projet Django de base
    create_django_project(&options.name)?;

    // Changer le répertoire de travail vers le nouveau projet
    std::env::set_current_dir(&options.name)?;

    // Créer la structure de base des applications
    create_apps_structure(options)?;

    // Créer la structure pour les tests
    create_tests_structure()?;

    // Créer la structure pour les configurations
    create_config_structure(options)?;

    // Créer la structure pour les assets statiques
    create_static_structure()?;

    // Créer la structure pour les templates
    create_templates_structure()?;

    // Créer la structure spécifique au type de projet
    create_project_type_structure(options)?;

    println!("Project structure created successfully.");
    Ok(())
}

fn create_django_project(project_name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("django-admin")
        .args(&["startproject", project_name])
        .status()?;
    Ok(())
}

fn create_apps_structure(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    fs::create_dir("apps")?;
    let common_apps = vec!["core", "users"];
    for app in common_apps {
        Command::new("python")
            .args(&["manage.py", "startapp", app, &format!("apps/{}", app)])
            .status()?;
    }
    Ok(())
}

fn create_tests_structure() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("tests/unit")?;
    fs::create_dir("tests/integration")?;
    fs::write("tests/__init__.py", "")?;
    Ok(())
}

fn create_config_structure(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    fs::create_dir("config")?;
    let env_files = vec!["dev.env", "prod.env", "test.env"];
    for file in env_files {
        fs::write(format!("config/{}", file), "")?;
    }
    create_database_config(options)?;
    Ok(())
}

fn create_database_config(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    let db_config = match options.db_type.as_str() {
        "sqlite" => "DATABASES = {\n    'default': {\n        'ENGINE': 'django.db.backends.sqlite3',\n        'NAME': BASE_DIR / 'db.sqlite3',\n    }\n}",
        "postgres" => "DATABASES = {\n    'default': {\n        'ENGINE': 'django.db.backends.postgresql',\n        'NAME': 'your_db_name',\n        'USER': 'your_db_user',\n        'PASSWORD': 'your_db_password',\n        'HOST': 'localhost',\n        'PORT': '5432',\n    }\n}",
        "mysql" => "DATABASES = {\n    'default': {\n        'ENGINE': 'django.db.backends.mysql',\n        'NAME': 'your_db_name',\n        'USER': 'your_db_user',\n        'PASSWORD': 'your_db_password',\n        'HOST': 'localhost',\n        'PORT': '3306',\n    }\n}",
        _ => return Err("Unsupported database type".into()),
    };
    fs::write("config/database.py", db_config)?;
    Ok(())
}

fn create_static_structure() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("static/css")?;
    fs::create_dir("static/js")?;
    fs::create_dir("static/images")?;
    Ok(())
}

fn create_templates_structure() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("templates/base")?;
    fs::write("templates/base/base.html", "<!DOCTYPE html>\n<html>\n<head>\n    <title>{% block title %}{% endblock %}</title>\n</head>\n<body>\n    {% block content %}{% endblock %}\n</body>\n</html>")?;
    Ok(())
}

fn create_project_type_structure(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    match options.project_type.as_str() {
        "rest" => create_rest_structure(),
        "graphql" => create_graphql_structure(),
        "fullstack" => create_fullstack_structure(options),
        _ => Err("Unsupported project type".into()),
    }
}

fn create_rest_structure() -> Result<(), Box<dyn Error>> {
    fs::create_dir("api")?;
    fs::write("api/__init__.py", "")?;
    fs::write(
        "api/urls.py",
        "from django.urls import path\n\nurlpatterns = [\n    # Add your API endpoints here\n]",
    )?;
    fs::write("api/views.py", "# Add your API views here")?;
    Ok(())
}

fn create_graphql_structure() -> Result<(), Box<dyn Error>> {
    fs::create_dir("graphql")?;
    fs::write("graphql/__init__.py", "")?;
    fs::write("graphql/schema.py", "import graphene\n\nclass Query(graphene.ObjectType):\n    pass\n\nschema = graphene.Schema(query=Query)")?;
    Ok(())
}

fn create_fullstack_structure(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    if let Some(frontend) = &options.frontend_framework {
        match frontend.as_str() {
            "react" => setup_react(),
            "vue" => setup_vue(),
            "angular" => setup_angular(),
            _ => return Err("Unsupported frontend framework".into()),
        }?;
    }
    Ok(())
}

fn setup_react() -> Result<(), Box<dyn Error>> {
    Command::new("npx")
        .args(&["create-react-app", "frontend"])
        .status()?;
    Ok(())
}

fn setup_vue() -> Result<(), Box<dyn Error>> {
    Command::new("vue").args(&["create", "frontend"]).status()?;
    Ok(())
}

fn setup_angular() -> Result<(), Box<dyn Error>> {
    Command::new("ng").args(&["new", "frontend"]).status()?;
    Ok(())
}

fn generate_project_files(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    println!("Generating project files...");
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            ".gitignore",
            include_str!("../../../templates/setup/.gitignore.tpl"),
        ),
        (
            "Dockerfile",
            include_str!("../../../templates/setup/Dockerfile.tpl"),
        ),
        (
            "README.md",
            include_str!("../../../templates/setup/README.md.tpl"),
        ),
        (
            "project_structure.yml",
            include_str!("../../../templates/project/project_structure.tpl"),
        ),
    ])?;

    let mut context = tera::Context::new();
    context.insert("project_name", &options.name);
    context.insert("project_type", &options.project_type);
    context.insert("db_type", &options.db_type);
    context.insert("auth_system", &options.auth_system);

    // TODO: Generate files using Tera
    Ok(())
}

fn configure_settings(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    println!("Configuring settings.py...");
    // TODO: Implement settings.py configuration
    Ok(())
}

fn initialize_version_control(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    if options.additional_options.contains(&"Git".to_string()) {
        println!("Initializing Git repository...");
        Command::new("git").arg("init").output()?;
    }
    Ok(())
}

fn configure_frontend(options: &ProjectOptions) -> Result<(), Box<dyn Error>> {
    if let Some(framework) = &options.frontend_framework {
        println!("Configuring frontend with {}...", framework);
        // TODO: Implement frontend configuration
    }
    Ok(())
}

// Les fonctions check_and_install_python et check_and_install_pip restent inchangées

pub fn check_and_install_python() -> Result<String, Box<dyn std::error::Error>> {
    let python_commands = ["python", "python3", "py"];

    for cmd in python_commands.iter() {
        match Command::new(cmd).arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap_or("Unknown")
                    .to_string();
                return Ok(version);
            }
            _ => continue,
        }
    }

    // Python n'est pas installé
    println!("Python n'est pas installé. Veuillez l'installer manuellement.");

    if cfg!(target_os = "windows") {
        println!("Visitez https://www.python.org/downloads/windows/ pour télécharger Python pour Windows.");
    } else if cfg!(target_os = "macos") {
        println!("Vous pouvez installer Python via Homebrew en exécutant : brew install python");
        println!(
            "Ou visitez https://www.python.org/downloads/mac-osx/ pour une installation manuelle."
        );
    } else if cfg!(target_os = "linux") {
        println!(
            "Utilisez le gestionnaire de paquets de votre distribution pour installer Python."
        );
        println!("Par exemple, sur Ubuntu ou Debian : sudo apt-get install python3");
    } else {
        println!(
            "Visitez https://www.python.org/downloads/ pour télécharger Python pour votre système."
        );
    }

    let _ = Confirm::new()
        .with_prompt("Appuyez sur Entrée une fois Python installé")
        .interact()?;
    check_and_install_python()
}

pub fn check_and_install_pip(python_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pip_command = if python_version.starts_with("3") {
        "pip3"
    } else {
        "pip"
    };

    match Command::new(pip_command).arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!(
                "pip est installé. ({})",
                String::from_utf8_lossy(&output.stdout).trim()
            );
            return Ok(());
        }
        _ => {
            println!("pip n'est pas installé. Installation en cours...");

            if cfg!(target_os = "windows") {
                // Sur Windows, pip est généralement inclus avec Python
                println!("pip devrait être inclus avec votre installation Python.");
                println!("Si ce n'est pas le cas, essayez de réinstaller Python en cochant l'option 'Add Python to PATH'.");
            } else {
                // Sur macOS et Linux, on peut utiliser get-pip.py
                let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";

                // Télécharger get-pip.py
                Command::new("curl").args(&["-O", get_pip_url]).status()?;

                // Exécuter get-pip.py
                let python_cmd = if python_version.starts_with("3") {
                    "python3"
                } else {
                    "python"
                };
                Command::new(python_cmd).arg("get-pip.py").status()?;

                // Supprimer get-pip.py
                if cfg!(target_os = "windows") {
                    Command::new("del").arg("get-pip.py").status()?;
                } else {
                    Command::new("rm").arg("get-pip.py").status()?;
                }
            }

            // Vérifier à nouveau l'installation de pip
            match Command::new(pip_command).arg("--version").output() {
                Ok(output) if output.status.success() => {
                    println!("pip a été installé avec succès.");
                    Ok(())
                }
                _ => Err("Échec de l'installation de pip.".into()),
            }
        }
    }
}
