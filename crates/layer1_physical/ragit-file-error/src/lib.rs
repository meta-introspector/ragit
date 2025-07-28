use std::ffi::OsString;
use std::fmt;
use std::io;
use thiserror::Error;

#[derive(Clone, PartialEq, Error)]
pub struct FileError {
    pub kind: FileErrorKind,
    pub given_path: Option<String>,
}

impl FileError {
    pub fn from_std(e: io::Error, given_path: &str) -> Self {
        let kind = match e.kind() {
            io::ErrorKind::NotFound => FileErrorKind::FileNotFound,
            io::ErrorKind::PermissionDenied => FileErrorKind::PermissionDenied,
            io::ErrorKind::AlreadyExists => FileErrorKind::AlreadyExists,
            e => FileErrorKind::Unknown(format!("unknown error: {e:?}")),
        };

        FileError {
            kind,
            given_path: Some(given_path.to_string()),
        }
    }

    pub fn os_str_err(os_str: OsString) -> Self {
        FileError {
            kind: FileErrorKind::OsStrErr(os_str),
            given_path: None,
        }
    }

    pub fn cannot_diff_path(path: String, base: String) -> Self {
        FileError {
            kind: FileErrorKind::CannotDiffPath(path.to_string(), base),
            given_path: Some(path),
        }
    }

    pub fn unknown(msg: String, path: Option<String>) -> Self {
        FileError {
            kind: FileErrorKind::Unknown(msg),
            given_path: path,
        }
    }
}

impl fmt::Debug for FileError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind)
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind)
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum FileErrorKind {
    #[error("file not found")]
    FileNotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("file already exists")]
    AlreadyExists,
    #[error("cannot calc diff: `{0}` and `{1}`")]
    CannotDiffPath(String, String),
    #[error("unknown file error: `{0}`")]
    Unknown(String),
    #[error("error converting os_str: `{0:?}`")]
    OsStrErr(OsString),
}

impl From<FileError> for io::Error {
    fn from(err: FileError) -> Self {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}