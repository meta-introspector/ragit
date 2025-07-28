use crate::{ArgCount, ArgType, Span};

pub struct Error {
    pub span: Span,
    pub kind: ErrorKind,
}

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
                (Some(min), Some(max)) => format!("N is supposed to be between {min} and {max}, but is {n}."),
                (Some(min), None) => format!("N is supposed to be at least {min}, but is {n}."),
                (None, Some(max)) => format!("N is supposed to be at most {max}, but is {n}."),
                (None, None) => unreachable!(),
            },
            ErrorKind::SameFlagMultipleTimes(prev, next) => if prev == next {
                format!("Flag `{next}` cannot be used multiple times.")
            } else {
                format!("Flag `{prev}` and `{next}` cannot be used together.")
            },
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
