use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    // URL of the file to fetch
    let url = "https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml";

    // Path to save the fetched file in the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("regexes.yaml");

    // Fetch the file using `curl` or `reqwest`
    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo:rerun-if-changed=templates"); // Re-run if any files in templates change

    if let Err(e) = fetch_file(url, &dest_path) {
        eprintln!("Failed to fetch the file: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = compile_css_files() {
        eprintln!("Failed to compile CSS files: {}", e);
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
