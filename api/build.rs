use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let migrations_dir = crate_dir.join("..").join("core").join("migrations");

    #[cfg(debug_assertions)]
    let db_file = crate_dir.join("development.db");
    #[cfg(not(debug_assertions))]
    let db_file = crate_dir.join("api_victorhqc_com.db");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo:rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    #[cfg(debug_assertions)]
    {
        build_aws_keys("DEVELOPMENT_");
    }
    #[cfg(not(debug_assertions))]
    {
        build_aws_keys("PRODUCTION_");
    }

    if let Err(e) = build_api_db(&db_file, &migrations_dir) {
        eprintln!("Failed to build DB: {}", e);
        std::process::exit(1);
    }
}

fn is_sqlx_offline() -> bool {
    std::env::var("SQLX_OFFLINE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false)
}

fn build_api_db(db_file: &Path, migrations_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = format!("sqlite:{}", db_file.display());

    if is_sqlx_offline() {
        println!("=== SQLX Offline Mode ===");
        println!("Skipping database creation and migrations");
        println!("Using prepared query metadata from .sqlx directory");
        println!("cargo:rustc-env=ROCKET_DATABASE_URL={}", db_url);
        println!("cargo:rustc-env=DATABASE_URL={}", db_url);
        return Ok(());
    }

    println!("=== Database Setup ===");
    println!("DB file: {}", db_file.display());
    println!("Migrations dir: {}", migrations_dir.display());
    println!("DB URL: {}", db_url);

    if !db_file.exists() {
        println!("Creating database...");

        let output = Command::new("sqlx")
            .arg("database")
            .arg("create")
            .arg("--database-url")
            .arg(&db_url)
            .output()?;

        if !output.status.success() {
            eprintln!("=== SQLX DATABASE CREATE FAILED ===");
            eprintln!("Command: sqlx database create --database-url {}", db_url);
            eprintln!("Exit status: {}", output.status);
            eprintln!("\n--- stdout ---");
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("\n--- stderr ---");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("=================================");
            return Err(
                format!("sqlx database create failed with status: {}", output.status).into(),
            );
        }

        println!("✓ Database created successfully");
    } else {
        println!("Database already exists");
    }

    println!("Running migrations...");

    let output = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .arg("--source")
        .arg(migrations_dir)
        .arg("--database-url")
        .arg(&db_url)
        .output()?;

    if !output.status.success() {
        eprintln!("=== SQLX MIGRATE FAILED ===");
        eprintln!(
            "Command: sqlx migrate run --source {} --database-url {}",
            migrations_dir.display(),
            db_url
        );
        eprintln!("Exit status: {}", output.status);
        eprintln!("\n--- stdout ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("\n--- stderr ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("===========================");
        return Err(format!("sqlx migrate failed with status: {}", output.status).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        println!("{}", stdout);
    }

    println!("✓ Migrations completed successfully");
    println!("✓ SQLite DB ready at {}", db_file.display());

    println!("cargo:rustc-env=ROCKET_DATABASE_URL={}", db_url);
    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    Ok(())
}

fn build_aws_keys(prefix: &str) {
    let access_key =
        std::env::var(format!("{}AWS_ACCESS_KEY_ID", prefix)).unwrap_or("".to_string());
    println!("cargo:rustc-env=AWS_ACCESS_KEY_ID={}", access_key);

    let secret_access_key =
        std::env::var(format!("{}AWS_SECRET_ACCESS_KEY", prefix)).unwrap_or("".to_string());
    println!(
        "cargo:rustc-env=AWS_SECRET_ACCESS_KEY={}",
        secret_access_key
    );

    let region = std::env::var(format!("{}AWS_REGION", prefix)).unwrap_or("".to_string());
    println!("cargo:rustc-env=AWS_REGION={}", region);

    let bucket_name = std::env::var(format!("{}AWS_BUCKET_NAME", prefix)).unwrap_or("".to_string());
    println!("cargo:rustc-env=AWS_BUCKET_NAME={}", bucket_name);
}
