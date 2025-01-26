#[cfg(not(debug_assertions))]
use flate2::{write::GzEncoder, Compression};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(not(debug_assertions))]
use std::{
    fs::File,
    io::{copy, BufReader, Write},
};
#[cfg(not(debug_assertions))]
use walkdir::WalkDir;

fn main() {
    let regex_target = Path::new("regexes.yaml");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let migrations_dir = crate_dir.join("migrations");

    // Only run this for release builds.
    #[cfg(not(debug_assertions))]
    let include_in_zip: Vec<(String, PathBuf)> = {
        let templates_dir = crate_dir.join("templates");
        let static_dir = crate_dir.join("static");
        let public_dir = crate_dir.join("public");

        println!("cargo:rerun-if-changed={}", templates_dir.display()); // Re-run if any files in templates change
        println!("cargo:rerun-if-changed={}", static_dir.display()); // Re-run if files in static change
        println!("cargo:rerun-if-changed={}", public_dir.display()); // Re-run if files in public change

        vec![
            ("templates".to_string(), templates_dir),
            ("static".to_string(), static_dir),
            ("public".to_string(), public_dir),
        ]
    };

    let db_file = crate_dir.join("analytics.db");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo:rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    if let Err(e) = fetch_file(regex_target) {
        eprintln!("Failed to fetch the file: {}", e);
        std::process::exit(1);
    }

    #[cfg(not(debug_assertions))]
    if let Err(e) = compile_css_files() {
        eprintln!("Failed to compile CSS files: {}", e);
        std::process::exit(1);
    }

    #[cfg(not(debug_assertions))]
    if let Err(e) = gzip_statics() {
        eprintln!("Failed to compress gzip files: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = build_analytics_db(&db_file, &migrations_dir) {
        eprintln!("Failed to build DB: {}", e);
        std::process::exit(1);
    }

    #[cfg(not(debug_assertions))]
    {
        if let Err(e) = compress_web_files(include_in_zip) {
            eprintln!("Failed to compress templates: {}", e);
            std::process::exit(1);
        }
    }
}

fn fetch_file(dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml";

    let response = reqwest::blocking::get(url)?.text()?;

    fs::write(dest_path, response)?;
    println!("Saved regex {:?}", dest_path);
    Ok(())
}

#[cfg(not(debug_assertions))]
fn compile_css_files() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let static_path = crate_dir.join("static");
    if !fs::exists(&static_path)? {
        fs::create_dir_all(&static_path)?;
    }

    let ext = std::ffi::OsStr::new("css");

    let mut combined_css = String::new();

    let templates_dir = crate_dir.join("templates");
    for entry in WalkDir::new(&templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(ext))
    {
        let content = fs::read_to_string(entry.path())?;
        combined_css.push_str(&content);
        combined_css.push('\n');
    }

    let temp_css = crate_dir.join("static").join("__tmp.css");
    let target_css = crate_dir.join("static").join("styles.min.css");
    fs::write(&temp_css, combined_css)?;

    let status = Command::new("npx")
        .args([
            "tailwindcss@v3",
            "-i",
            temp_css
                .as_os_str()
                .to_str()
                .expect("Failed to build __tmp.css path"),
            "-o",
            target_css
                .as_os_str()
                .to_str()
                .expect("Failed to build styles.min.css path"),
            "--minify",
        ])
        .status()?;

    fs::remove_file(temp_css)?;

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

    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    Ok(())
}

#[cfg(not(debug_assertions))]
fn gzip_statics() -> Result<(), Box<dyn std::error::Error>> {
    let css = std::ffi::OsStr::new("css");
    let js = std::ffi::OsStr::new("js");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let static_path = crate_dir.join("static");

    for entry in WalkDir::new(&static_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(css) || e.path().extension() == Some(js))
    {
        let file = File::open(entry.path())?;
        let mut buffer = BufReader::new(file);

        let new_extension = format!("{}.gz", entry.path().extension().unwrap().to_str().unwrap());
        let name = Path::new(entry.file_name()).with_extension(new_extension);
        eprintln!("filename output {:?}", name);
        let compressed = File::create(static_path.join(name))?;

        let mut encoder = GzEncoder::new(compressed, Compression::default());
        copy(&mut buffer, &mut encoder)?;
        encoder.finish()?;
    }

    Ok(())
}

/**
This function compresses the files that are needed to deploy alongside the binaries into the server.
These are usually static files like templates or other things that the web server will, well, serve.
*/
#[cfg(not(debug_assertions))]
fn compress_web_files(dirs: Vec<(String, PathBuf)>) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create("web_files.zip")?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for (prefix, dir) in dirs.iter() {
        zip.add_directory(prefix, options)?;

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // No need to handle directories, as WalkDir is traversing this for us.
            if path.is_dir() || path.file_name().map_or(false, |name| name == ".DS_Store") {
                continue;
            }

            let relative_path = path.strip_prefix(dir)?;

            let zip_path = PathBuf::from(prefix)
                .join(relative_path)
                .to_str()
                .ok_or("Invalid path")?
                .to_string();

            println!("Adding file to web_files.zip: {} ", zip_path);

            zip.start_file(zip_path, options)?;
            let mut f = std::fs::File::open(path)?;
            buffer.clear();
            std::io::Read::read_to_end(&mut f, &mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    println!("web_files.zip is Done");

    Ok(())
}
