use crate::cli::setup::{env, poetry, docker, git};
use crate::utils::tools::get_compatible_django_version;
use clap::Args;

#[derive(Args, Debug)]
pub struct SetupEnv {
    #[arg(short = 'p', long = "python-version", default_value = "3.9")]
    python_version: String,

    #[arg(short = 'v', long = "venv-name", default_value = "venv")]
    venv_name: String,

    #[arg(long)]
    with_poetry: bool,

    #[arg(long)]
    with_docker: bool,

    #[arg(long)]
    with_git: bool,
}

impl SetupEnv {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        env::check_and_install_python()?;
        env::create_virtual_env(&self.venv_name)?;
        env::install_dependencies(&self.venv_name, &self.python_version)?;

        if self.with_poetry {
            poetry::setup()?;
        }

        if self.with_docker {
            docker::generate_dockerfile(&self.python_version)?;
        }

        if self.with_git {
            git::init()?;
            git::create_gitignore(&self.venv_name)?;
        }

        // Utiliser la version de Django obtenue par `get_compatible_django_version`
        if let Some(django_version) = get_compatible_django_version(&self.python_version) {
            env::create_readme(&self.python_version, &self.venv_name, django_version)?;
        } else {
            println!("Aucune version compatible de Django trouvée pour la version de Python.");
        }

        println!("Configuration de l'environnement terminée avec succès !");

        Ok(())
    }
}
