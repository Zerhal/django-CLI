use clap::{Arg, Command, ArgMatches};

pub fn init_command() -> Command {
    Command::new("init")
        .about("Initialize a new Django project")
        .arg(
            Arg::new("project_name")
                .help("Sets the name of the project")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("backend")
                .short('b')
                .long("backend")
                .help("Specify the backend framework"),
        )
        .arg(
            Arg::new("frontend")
                .short('f')
                .long("frontend")
                .help("Specify the frontend framework"),
        )
}

pub fn handle_init(args: &ArgMatches) {
    let project_name = args.get_one::<String>("project_name").map(|s| s.as_str()).unwrap_or("default_project");
    let frontend = args.get_one::<String>("frontend").map(|s| s.as_str()).unwrap_or("N/A");
    println!("Initializing Django project: {}", project_name);
    println!("Frontend: {}", frontend);
    // Logique d'initialisation ici
}
