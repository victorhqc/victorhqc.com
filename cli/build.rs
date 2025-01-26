use std::path::PathBuf;
fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-changed=build.rs"); // Re-run build if build.rs changes
    #[cfg(debug_assertions)]
    let db_file = crate_dir
        .parent()
        .unwrap()
        .join("api")
        .join("development.db");
    #[cfg(not(debug_assertions))]
    let db_file = crate_dir
        .parent()
        .unwrap()
        .join("api")
        .join("api_victorhqc_com.db");
    let db_url = format!("sqlite:{}", db_file.display());
    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    #[cfg(debug_assertions)]
    {
        build_aws_keys("DEVELOPMENT_");
    }
    #[cfg(not(debug_assertions))]
    {
        build_aws_keys("PRODUCTION_");
    }
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
