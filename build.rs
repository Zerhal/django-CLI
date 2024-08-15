use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("templates");
    fs::create_dir_all(&dest_path).unwrap();

    fs::copy("templates/project/project_structure.tpl", dest_path.join("project_structure.tpl")).unwrap();

    // Copy templates for setup
    fs::copy("templates/setup/README.md.tpl", dest_path.join("README.md.tpl")).unwrap();
    fs::copy("templates/setup/.gitignore.tpl", dest_path.join(".gitignore.tpl")).unwrap();
    fs::copy("templates/setup/Dockerfile.tpl", dest_path.join("Dockerfile.tpl")).unwrap();

    println!("cargo:rerun-if-changed=templates/setup");
}
