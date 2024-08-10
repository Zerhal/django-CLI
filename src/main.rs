mod commands;
mod config;
mod django;
mod utils;

use config::get_project_config;
use django::{configure_django_project, create_django_project, install_dependencies};
use utils::print_metadata;

fn main() {
    // Afficher les métadonnées en haut du terminal
    print_metadata();

    // Obtenir la configuration du projet à partir des arguments ou des invites utilisateur
    let project_config = get_project_config();

    // Créer le projet Django
    create_django_project(&project_config.project_name);

    // Configurer le projet Django
    configure_django_project(&project_config);

    // Installer les dépendances
    install_dependencies(&project_config.project_name);

    println!("Projet Django configuré avec succès !");
}
