use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("zgrep: FILE ERROR: {} : {}", ._0, ._1)]
    FileError(String, &'static str),
    #[error("zgrep: ARGUMENT ERROR: Expected {} but received {} arguments", ._0, ._1)]
    ArgError(usize, usize),
    #[error("zgrep: OPTION ERROR: Unknown option: {}", ._0)]
    OptionError(String),
    #[error("zgrep: PATH ERROR: Path is not valid UTF-8: {:?}", ._0)]
    PathError(PathBuf),
    #[error("zgrep: DIRECTORY ENTRY ERROR")]
    EntryError,
}
