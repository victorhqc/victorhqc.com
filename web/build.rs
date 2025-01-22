use std::fs;
#[cfg(not(debug_assertions))]
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let regex_target = Path::new("regexes.yaml");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let migrations_dir = crate_dir.join("migrations");

    #[cfg(not(debug_assertions))]
    let templates_dir = crate_dir.join("templates");
    #[cfg(not(debug_assertions))]
    let static_dir = crate_dir.join("static");
    #[cfg(not(debug_assertions))]
    let public_dir = crate_dir.join("public");

    let db_file = crate_dir.join("analytics.db");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo::rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    // Only run this for release builds.
    #[cfg(not(debug_assertions))]
    {
        println!("cargo:rerun-if-changed={}", templates_dir.display()); // Re-run if any files in templates change
        println!("cargo::rerun-if-changed={}", static_dir.display()); // Re-run if files in static change
        println!("cargo::rerun-if-changed={}", public_dir.display()); // Re-run if files in public change
    }

    if let Err(e) = fetch_file(regex_target) {
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

    #[cfg(not(debug_assertions))]
    {
        if let Err(e) = compress_static(&templates_dir, "templates") {
            eprintln!("Failed to compress templates: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = compress_static(&public_dir, "public") {
            eprintln!("Failed to compress public: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = compress_static(&static_dir, "static") {
            eprintln!("Failed to compress static: {}", e);
            std::process::exit(1);
        }
    }
}

fn fetch_file(dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml";

    let response = reqwest::blocking::get(url)?.text()?;

    println!("Attempting to save file at {:?}", dest_path);
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

/**
This function compresses the files that are needed to deploy alongside the binaries into the server.
These are usually static files like templates or other things that the web server will, well, serve.
*/
#[cfg(not(debug_assertions))]
fn compress_static(dir: &PathBuf, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(format!("{}.zip", file_name))?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(dir)?.to_str().ok_or("Invalid path")?;

        // No need to handle directories, as WalkDir is traversing this for us.
        if path.is_dir() {
            continue;
        }

        println!("Adding file to {}.zip: {} ", file_name, name);

        zip.start_file(name, options)?;
        let mut f = std::fs::File::open(path)?;
        buffer.clear();
        std::io::Read::read_to_end(&mut f, &mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    println!("{}.zip is Done", file_name);

    Ok(())
}
