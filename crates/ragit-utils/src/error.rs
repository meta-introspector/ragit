use ragit_error::ApiError;
pub use ragit_fs::FileError;
pub use ragit_pdl::JsonType;
use ragit_types::uid::Uid;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    /// see <https://doc.rust-lang.org/stable/std/num/struct.ParseIntError.html>
    ParseIntError(std::num::ParseIntError),

    IntegerNotInRange {
        min: Option<i128>,
        max: Option<i128>,
        n: i128,
    },

    /// (prev_flag, curr_flag)
    SameFlagMultipleTimes(String, String),

    /// of an arg_flag
    MissingArgument(String, ArgType),

    WrongArgCount {
        expected: ArgCount,
        got: usize,
    },
    MissingFlag(String),
    UnknownFlag {
        flag: String,
        similar_flag: Option<String>,
    },
}

impl ErrorKind {
    pub fn render(&self) -> String {
        match self {
            ErrorKind::ParseIntError(_) => String::from("Cannot parse int."),
            ErrorKind::IntegerNotInRange { min, max, n } => match (min, max) {
                (Some(min), Some(max)) => {
                    format!("N is supposed to be between {min} and {max}, but is {n}.")
                }
                (Some(min), None) => format!("N is supposed to be at least {min}, but is {n}."),
                (None, Some(max)) => format!("N is supposed to be at most {max}, but is {n}."),
                (None, None) => unreachable!(),
            },
            ErrorKind::SameFlagMultipleTimes(prev, next) => {
                if prev == next {
                    format!("Flag `{next}` cannot be used multiple times.")
                } else {
                    format!("Flag `{prev}` and `{next}` cannot be used together.")
                }
            }
            ErrorKind::MissingArgument(arg, arg_type) => format!(
                "A {} value is required for flag `{arg}`, but is missing.",
                format!("{arg_type:?}").to_ascii_lowercase(),
            ),
            ErrorKind::WrongArgCount { expected, got } => format!(
                "Expected {} arguments, got {got} arguments",
                match expected {
                    ArgCount::Exact(n) => format!("exactly {n}"),
                    ArgCount::Geq(n) => format!("at least {n}"),
                    ArgCount::Leq(n) => format!("at most {n}"),
                    ArgCount::None => String::from("no"),
                    ArgCount::Any => unreachable!(),
                },
            ),
            ErrorKind::MissingFlag(flag) => format!("Flag `{flag}` is missing."),
            ErrorKind::UnknownFlag { flag, similar_flag } => format!(
                "Unknown flag: `{flag}`.{}",
                if let Some(flag) = similar_flag {
                    format!(" There is a similar flag: `{flag}`.")
                } else {
                    String::new()
                },
            ),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

use crate::cli_types::{ArgCount, ArgType};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("json type error: {expected}, got {got}")]
    JsonTypeError { expected: JsonType, got: JsonType },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonSerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("from utf8 error")]
    FromUtf8Error,
    #[error(transparent)]
    ApiError(#[from] ApiError),
    
    #[error(transparent)]
    PdlError(#[from] ragit_pdl::Error),
    #[error(transparent)]
    PdlSchemaParseError(#[from] ragit_pdl::SchemaParseError),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error("broken inverted index: {0}")]
    BrokenII(String),
    #[error("no such chunk: {0}")]
    NoSuchChunk(Uid),
    #[error("no such file: {path:?}, {uid:?}")]
    NoSuchFile {
        path: Option<PathBuf>,
        uid: Option<Uid>,
    },
    #[error("broken index: {0}")]
    BrokenIndex(String),
    #[error("invalid config key: {0}")]
    InvalidConfigKey(String),
    #[error("prompt missing: {0}")]
    PromptMissing(String),
    #[error("model not selected")]
    ModelNotSelected,
    #[error("index already exists: {0}")]
    IndexAlreadyExists(PathBuf),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    CliError(#[from] CliError),
    
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("parse int error: {0}")]
    ParseIntError(std::num::ParseIntError, Span),
    #[error("integer not in range: {n} not between {min:?} and {max:?}")]
    IntegerNotInRange {
        min: Option<i128>,
        max: Option<i128>,
        n: i128,
        span: Span,
    },
    #[error("same flag multiple times: {0}")]
    SameFlagMultipleTimes(String, String, Span),
    #[error("missing argument: {0}")]
    MissingArgument(String, ArgType, Span),
    #[error("wrong arg count: expected {expected}, got {got}")]
    WrongArgCount {
        expected: ArgCount,
        got: usize,
        span: Span,
    },
    #[error("missing flag: {0}")]
    MissingFlag(String, Span),
    #[error("unknown flag: {flag}, similar flag: {similar_flag:?}")]
    UnknownFlag {
        flag: String,
        similar_flag: Option<String>,
        span: Span,
    },
}

impl CliError {
    pub fn new_message(message: String) -> Self {
        CliError::MissingArgument(message, ArgType::String, Span::End)
    }

    pub fn new_message_with_span(message: String, span: Span) -> Self {
        CliError::MissingArgument(message, ArgType::String, span)
    }

    pub fn get_span(&self) -> Span {
        match self {
            CliError::ParseIntError(_, span) => span.clone(),
            CliError::IntegerNotInRange { span, .. } => span.clone(),
            CliError::SameFlagMultipleTimes(_, _, span) => span.clone(),
            CliError::MissingArgument(_, _, span) => span.clone(),
            CliError::WrongArgCount { span, .. } => span.clone(),
            CliError::MissingFlag(_, span) => span.clone(),
            CliError::UnknownFlag { span, .. } => span.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Span {
    Exact(usize),
    FirstArg,
    End,
    NthArg(usize),
    Rendered((String, usize, usize)),
}

impl Span {
    pub fn render(&self, args: &[String], skip_first_n: usize) -> Self {
        let mut rendered_args = Vec::with_capacity(args.len());
        let mut arg_indices = vec![];

        for (index, arg) in args.iter().enumerate() {
            if !arg.starts_with("--") && index >= skip_first_n {
                arg_indices.push(index);
            }

            if arg.contains(" ") || arg.contains("\"") || arg.contains("'") || arg.contains("\n") {
                rendered_args.push(format!("{:?}", arg));
            } else {
                rendered_args.push(arg.to_string());
            }
        }

        let new_span = match self {
            Span::Exact(n) => Span::Exact(*n),
            Span::FirstArg => match arg_indices.get(0) {
                Some(n) => Span::Exact(*n),
                None => Span::End,
            },
            Span::NthArg(n) => match arg_indices.get(*n) {
                Some(n) => Span::Exact(*n),
                None => Span::End,
            },
            _ => self.clone(),
        };
        let selected_index = match new_span {
            Span::Exact(n) => n,
            _ => 0,
        };
        let mut joined_args = rendered_args.join(" ");
        let (start, end) = if joined_args.is_empty() {
            joined_args = String::from(" ");
            (0, 1)
        } else {
            // append a whitespace so that `Span::End` is more readable
            joined_args = format!("{} ", joined_args);

            match new_span {
                Span::End => (joined_args.len() - 1, joined_args.len()),
                _ => (
                    rendered_args[..selected_index]
                        .iter()
                        .map(|arg| arg.len())
                        .sum::<usize>()
                        + selected_index,
                    rendered_args[..(selected_index + 1)]
                        .iter()
                        .map(|arg| arg.len())
                        .sum::<usize>()
                        + selected_index,
                ),
            }
        };

        Span::Rendered((joined_args, start, end))
    }

    pub fn unwrap_rendered(&self) -> (String, usize, usize) {
        match self {
            Span::Rendered((span, start, end)) => (span.to_string(), *start, *end),
            _ => panic!(),
        }
    }
}

pub fn underline_span(args: &str, start: usize, end: usize) -> String {
    format!(
        "{}
{}{}{}",
        args,
        " ".repeat(start),
        "^".repeat(end - start),
        " ".repeat(args.len() - end),
    )
}

impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> Self {
        CliError::ParseIntError(err, Span::End)
    }
}
