use crate::commands::setup_command;
use crate::utils::tools::{get_project_name, sanitize_project_name, select_option};

#[allow(dead_code)]
pub struct ProjectConfig {
    pub project_name: String,
    pub project_type: String,
    pub frontend_type: String,
    pub api_type: String,
    pub db_type: String,
    pub auth_enabled: String,
    pub roles: String,
    pub payment: String,
    pub email_service: String,
    pub cache_system: String,
    pub testing: String,
    pub deployment: String,
}

pub fn get_project_config() -> ProjectConfig {
    let matches = setup_command().get_matches();

    // Demander le nom du projet
    let mut project_name = get_project_name();
    project_name = sanitize_project_name(&project_name);

    let project_type = matches
        .get_one::<String>("project_type")
        .map(|s| s.to_string())
        .unwrap_or_else(|| select_option("Type de projet", &["Backend", "Fullstack"]));

    let frontend_type = if project_type == "Fullstack" {
        matches
            .get_one::<String>("frontend_type")
            .map(|s| s.to_string())
            .unwrap_or_else(|| select_option("Type de frontend", &["Django Template", "React"]))
    } else {
        "N/A".to_string()
    };

    let api_type = matches
        .get_one::<String>("api_type")
        .map(|s| s.to_string())
        .unwrap_or_else(|| select_option("Type d'API", &["Aucune", "REST", "GraphQL"]));

    let db_type = matches
        .get_one::<String>("db_type")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            select_option(
                "Type de base de données",
                &["SQLite", "PostgreSQL", "MySQL"],
            )
        });

    let auth_enabled = select_option(
        "Souhaitez-vous inclure un système d'authentification des utilisateurs ?",
        &["Oui", "Non"],
    );
    let roles = if auth_enabled == "Oui" {
        select_option(
            "Combien de rôles utilisateurs souhaitez-vous gérer ?",
            &[
                "1 (Utilisateur standard)",
                "2 (Utilisateur et Admin)",
                "Plus (rôles personnalisés)",
            ],
        )
    } else {
        "Aucun".to_string()
    };

    let payment = matches
        .get_one::<String>("payment")
        .map(|s| s.to_string())
        .unwrap_or_else(|| select_option("Intégrer un système de paiement", &["Oui", "Non"]));

    let email_service = matches
        .get_one::<String>("email_service")
        .map(|s| s.to_string())
        .unwrap_or_else(|| select_option("Intégrer un service d'envoi de mails", &["Oui", "Non"]));

    let cache_system = select_option(
        "Souhaitez-vous configurer un système de cache pour améliorer les performances ?",
        &["Oui", "Non"],
    );

    let testing = select_option(
        "Souhaitez-vous inclure un système de tests automatisés dès le départ ?",
        &["Oui", "Non"],
    );

    let deployment = select_option(
        "Comment souhaitez-vous déployer votre application ?",
        &["Docker", "Configuration manuelle (serveur)"],
    );

    ProjectConfig {
        project_name,
        project_type,
        frontend_type,
        api_type,
        db_type,
        auth_enabled,
        roles,
        payment,
        email_service,
        cache_system,
        testing,
        deployment,
    }
}
