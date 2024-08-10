use clap::{Arg, Command};

pub fn setup_command() -> Command {
    Command::new("Django Boilerplate Setup")
        .version("1.0")
        .author(
            r"
            .------..------..------..------..------..------.
            |Z.--. ||E.--. ||R.--. ||H.--. ||A.--. ||L.--. |
            | :(): || (\/) || :(): || :/\: || (\/) || :/\: |
            | ()() || :\/: || ()() || (__) || :\/: || (__) |
            | '--'Z|| '--'E|| '--'R|| '--'H|| '--'A|| '--'L|
            `------'`------'`------'`------'`------'`------'
            ",
        )
        .about("Configure votre projet Django")
        .arg(
            Arg::new("project_type")
                .short('t')
                .long("type")
                .value_name("PROJECT_TYPE")
                .help("Type de projet: Backend ou Fullstack"),
        )
        .arg(
            Arg::new("frontend_type")
                .short('f')
                .long("frontend")
                .value_name("FRONTEND_TYPE")
                .help("Type de frontend: Django Template ou React")
                .requires_if("Fullstack", "project_type"),
        )
        .arg(
            Arg::new("api_type")
                .short('a')
                .long("api")
                .value_name("API_TYPE")
                .help("Type d'API: Aucune, REST ou GraphQL"),
        )
        .arg(
            Arg::new("db_type")
                .short('d')
                .long("database")
                .value_name("DB_TYPE")
                .help("Type de base de données: SQLite, PostgreSQL, MySQL"),
        )
        .arg(
            Arg::new("payment")
                .short('p')
                .long("payment")
                .value_name("PAYMENT")
                .help("Intégrer un système de paiement: Oui ou Non"),
        )
        .arg(
            Arg::new("email_service")
                .short('e')
                .long("email")
                .value_name("EMAIL_SERVICE")
                .help("Intégrer un service d'envoi de mails: Oui ou Non"),
        )
}
