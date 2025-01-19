use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let migrations_dir = crate_dir.join("..").join("core").join("migrations");

    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    println!("cargo::rerun-if-changed={}", migrations_dir.display()); // Re-run if migrations change

    #[cfg(debug_assertions)]
    let db_file = crate_dir.join("..").join("api").join("development.db");
    #[cfg(not(debug_assertions))]
    let db_file = crate_dir
        .join("..")
        .join("api")
        .join("api_victorhqc_com.db");

    let db_url = format!("sqlite:{}", db_file.display());
    println!("cargo::rustc-env=DATABASE_URL={}", db_url);
}
