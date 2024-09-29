use super::graph::RootSchema;
use snafu::prelude::*;
use std::{
    fs::File,
    io::{Error as IOError, Write},
    path::Path,
};

pub fn sdl_gen(schema: &RootSchema) -> Result<()> {
    let sdl = schema.sdl();
    let path = Path::new("src/graphql/schema.gql");

    let mut output = File::create(path).context(FileCreationSnafu)?;
    write!(output, "{}", sdl).context(SDLWriteSnafu)?;

    Ok(())
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to create SDL File {}", source))]
    FileCreationError { source: IOError },

    #[snafu(display("Failed to write the SDL"))]
    SDLWriteError { source: IOError },
}
