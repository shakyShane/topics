use core::convert::From;
use core::fmt::{Display, Formatter};
use core::option::Option;
use core::option::Option::Some;
use core::result::Result::Ok;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error(
    "could not read file `{}`\nFull path: {}",
    pb.display(),
    abs.display()
    )]
    PathRead {
        pb: PathBuf,
        abs: PathBuf,
        original: std::io::Error,
    },
    #[error(
        "{}",
        .0
    )]
    SerdeLocationErr(LocationError),
    #[error("{}", .0)]
    Unknown(String),
    #[error("File format not supported: {}", .0.display())]
    NotSupported(PathBuf),
}

impl From<anyhow::Error> for DocError {
    fn from(e: anyhow::Error) -> Self {
        DocError::Unknown(e.to_string())
    }
}

impl From<toml::de::Error> for DocError {
    fn from(e: toml::de::Error) -> Self {
        DocError::Unknown(e.to_string())
    }
}

#[derive(Debug)]
pub struct LocationError {
    pub location: Option<Location>,
    pub input_file: Option<PathBuf>,
    pub input_file_src: String,
    pub description: String,
}

#[derive(Debug)]
pub enum Location {
    LineAndCol {
        line: usize,
        column: usize,
    },
    LineAndColRegion {
        line_start: usize,
        line_end: usize,
        line: usize,
        column: usize,
    },
    Region {
        line_start: usize,
        line_end: usize,
    },
    // Unknown,
}

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f);
        if let Some(location) = &self.location {
            match location {
                Location::LineAndColRegion { line, column, .. }
                | Location::LineAndCol { line, column } => {
                    let _ = writeln!(f, "    msg: {}", self.description);
                    let _ = writeln!(
                        f,
                        "   file: {}",
                        self.input_file
                            .as_ref()
                            .map(|f| f.display().to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    if *line != 0 {
                        let _ = writeln!(f, "   line: {}", line);
                        let _ = writeln!(f, " column: {}", column);
                    }
                }
                Location::Region {
                    line_start,
                    line_end,
                } => {
                    let _ = writeln!(f, "           msg: {}", self.description);
                    let _ = writeln!(
                        f,
                        "          file: {}",
                        self.input_file
                            .as_ref()
                            .map(|f| f.display().to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    let _ = writeln!(f, " between lines: {} & {}", line_start, line_end);
                }
            }
        }
        Ok(())
    }
}
