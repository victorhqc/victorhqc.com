use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    // URL of the file to fetch
    let url = "https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml";

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let regex_target = Path::new(&out_dir).join("regexes.yaml");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let templates_dir = crate_dir.join("templates");

    let migrations_dir = crate_dir.join("migrations");
    let db_file = crate_dir.join("analytics.db");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo:rerun-if-changed={}", templates_dir.display()); // Re-run if any files in templates change
    println!("cargo::rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    if let Err(e) = fetch_file(url, &regex_target) {
        eprintln!("Failed to fetch the file: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = compile_css_files() {
        eprintln!("Failed to compile CSS files: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = build_analytics_db(&db_file, &migrations_dir) {
        eprintln!("Failed to build DB: {}", e);
        std::process::exit(1);
    }
}

fn fetch_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    fs::write(dest_path, response)?;
    Ok(())
}

fn compile_css_files() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("static")?;

    let ext = std::ffi::OsStr::new("css");

    let mut combined_css = String::new();
    for entry in WalkDir::new("templates")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(ext))
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
        let content = fs::read_to_string(entry.path())?;
        combined_css.push_str(&content);
        combined_css.push('\n');
    }

    let temp_input = "static/not_compiled.css";
    fs::write(temp_input, combined_css)?;

    let status = Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            temp_input,
            "-o",
            "static/styles.min.css",
            "--minify",
        ])
        .status()?;

    fs::remove_file(temp_input)?;

    if !status.success() {
        return Err("tailwindcss compilation failed".into());
    }

    Ok(())
}

fn build_analytics_db(
    db_file: &PathBuf,
    migrations_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = format!("sqlite:{}", db_file.display());

    if !db_file.exists() {
        std::fs::write(db_file, b"").unwrap();
    }

    let exit_status = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .arg("--source")
        .arg(migrations_dir)
        .arg("--database-url")
        .arg(&db_url)
        .spawn()
        .unwrap()
        .wait()?;

    if !exit_status.success() {
        panic!("sqlx failed: {exit_status}");
    }

    println!("SQLite DB created at {}", db_file.display());
    println!("DATABASE_URL={}", db_url);

    println!("cargo::rustc-env=DATABASE_URL={}", db_url);

    Ok(())
}
