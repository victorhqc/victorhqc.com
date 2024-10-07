use crate::exiftool::json::{Error as JsonError, ExifData, JsonValue};
use log::debug;
use snafu::prelude::*;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(target_os = "windows")]
use winapi::um::winbase::CREATE_NO_WINDOW;

const EXTENSIONS: [&str; 3] = ["jpg", "png", "jpeg"];

pub fn read_metadata(img_path: &Path) -> Result<Vec<ExifData>, Error> {
    let extension = img_path.extension().unwrap_or("none".as_ref());
    let extension = extension.to_str().unwrap_or("none").to_lowercase();

    if img_path.is_dir() || !EXTENSIONS.contains(&extension.as_str()) {
        return Err(Error::Path {
            path: img_path.to_str().unwrap_or("NONE").to_string(),
        });
    }

    let path = exiftool_path()?;
    debug!("Exiftool Dir {:?}", path);
    let mut cmd = spawn_exiftool(&path);

    #[cfg(not(target_os = "windows"))]
    let cmd = cmd.arg(path);

    let output = cmd
        .arg("-a")
        .arg("-m")
        .arg("-j")
        .arg(img_path)
        .output()
        .context(ExiftoolSnafu)?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        let json = json.as_array().unwrap().first().unwrap();
        let value = JsonValue(json.clone());
        let result: Vec<ExifData> = value.try_into().context(JsonSnafu)?;

        Ok(result)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        Err(Error::Stderr {
            stderr: stderr.to_string(),
        })
    }
}

fn exiftool_path() -> Result<PathBuf, Error> {
    let root = std::env::current_dir().context(CurrentDirSnafu)?;
    #[cfg(target_os = "windows")]
    let path = root.join("deps").join("exiftool").join("exiftool(-k).exe");

    #[cfg(not(target_os = "windows"))]
    let path = root.join("deps").join("exiftool").join("exiftool");

    Ok(path)
}

#[cfg(not(target_os = "windows"))]
pub fn spawn_exiftool(_: &Path) -> Command {
    Command::new("perl")
}

#[cfg(target_os = "windows")]
fn spawn_exiftool(exiftool_path: &Path) -> Command {
    let mut cmd = Command::new(exiftool_path);
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get current dir: {:?}", source))]
    CurrentDir { source: std::io::Error },

    #[snafu(display("Invalid File: {:?}", path))]
    Path { path: String },

    #[snafu(display("Failed to run exiftool: {:?}", source))]
    Exiftool { source: std::io::Error },

    #[snafu(display("Something went wrong while running exiftool: {}", stderr))]
    Stderr { stderr: String },

    #[snafu(display("Failed to parse JSON: {:?}", source))]
    Json { source: JsonError },
}
