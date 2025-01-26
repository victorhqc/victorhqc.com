use std::path::PathBuf;
use std::process::Command;

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let migrations_dir = crate_dir.join("migrations");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo:rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    let db_file = crate_dir.join("__DB_FOR_MACROS__.db");

    let db_url = format!("sqlite:{}", db_file.display());

    std::process::Command::new("sqlx")
        .arg("database")
        .arg("create")
        .env("DATABASE_URL", &db_url)
        .output()
        .unwrap();

    Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .arg("--source")
        .arg(&migrations_dir)
        .env("DATABASE_URL", &db_url)
        .output()
        .unwrap();

    println!("cargo:rustc-env=DATABASE_URL={}", db_url);
}
