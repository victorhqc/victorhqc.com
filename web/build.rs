use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // URL of the file to fetch
    let url = "https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml";

    // Path to save the fetched file in the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("regexes.yaml");

    // Fetch the file using `curl` or `reqwest`
    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    if let Err(e) = fetch_file(url, &dest_path) {
        eprintln!("Failed to fetch the file: {}", e);
        std::process::exit(1);
    }
}

fn fetch_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    fs::write(dest_path, response)?;
    Ok(())
}
