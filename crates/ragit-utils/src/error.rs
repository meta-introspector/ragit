use ragit_uid::Uid;
pub use ragit_api::Error as ApiError;
pub use ragit_pdl::JsonType;
pub use ragit_fs::FileError;
    
use std::path::PathBuf;
use crate::cli_types::{ArgCount, ArgType};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("json type error: {expected}, got {got}")]
    JsonTypeError {
        expected: JsonType,
        got: JsonType,
    },
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
    NoSuchFile { path: Option<PathBuf>, uid: Option<Uid> },
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
    UidError(#[from] ragit_uid::UidError),
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
            }

            else {
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
                    rendered_args[..selected_index].iter().map(|arg| arg.len()).sum::<usize>() + selected_index,
                    rendered_args[..(selected_index + 1)].iter().map(|arg| arg.len()).sum::<usize>() + selected_index,
                ),
            }
        };

        Span::Rendered((
            joined_args,
            start,
            end,
        ))
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
