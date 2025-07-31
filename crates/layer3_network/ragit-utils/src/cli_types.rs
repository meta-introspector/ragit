use crate::error::Error;
pub use crate::error::ErrorKind;
pub use crate::error::{CliError, Span};
use crate::string_utils::get_closest_string;
use std::collections::HashMap;

pub struct ArgParser {
    arg_count: ArgCount,
    arg_type: ArgType,
    flags: Vec<Flag>,
    aliases: HashMap<String, String>,

    // `--N=20`, `--prefix=rust`
    arg_flags: HashMap<String, ArgFlag>,

    // '-f' -> '--force'
    short_flags: HashMap<String, String>,
}

impl ArgParser {
    pub fn new() -> Self {
        ArgParser {
            arg_count: ArgCount::None,
            arg_type: ArgType::String,
            flags: vec![],
            aliases: HashMap::new(),
            arg_flags: HashMap::new(),
            short_flags: HashMap::new(),
        }
    }

    pub fn args(&mut self, arg_type: ArgType, arg_count: ArgCount) -> &mut Self {
        self.arg_type = arg_type;
        self.arg_count = arg_count;
        self
    }

    pub fn flag(&mut self, flags: &[&str]) -> &mut Self {
        self.flags.push(Flag {
            values: flags.iter().map(|flag| flag.to_string()).collect(),
            optional: false,
            default: None,
        });
        self
    }

    pub fn optional_flag(&mut self, flags: &[&str]) -> &mut Self {
        self.flags.push(Flag {
            values: flags.iter().map(|flag| flag.to_string()).collect(),
            optional: true,
            default: None,
        });
        self
    }

    pub fn arg_flag(&mut self, flag: &str, arg_type: ArgType) -> &mut Self {
        self.arg_flags.insert(
            flag.to_string(),
            ArgFlag {
                flag: flag.to_string(),
                optional: false,
                default: None,
                arg_type,
            },
        );
        self
    }

    pub fn optional_arg_flag(&mut self, flag: &str, arg_type: ArgType) -> &mut Self {
        self.arg_flags.insert(
            flag.to_string(),
            ArgFlag {
                flag: flag.to_string(),
                optional: true,
                default: None,
                arg_type,
            },
        );
        self
    }

    pub fn arg_flag_with_default(
        &mut self,
        flag: &str,
        default: &str,
        arg_type: ArgType,
    ) -> &mut Self {
        self.arg_flags.insert(
            flag.to_string(),
            ArgFlag {
                flag: flag.to_string(),
                optional: true,
                default: Some(default.to_string()),
                arg_type,
            },
        );
        self
    }

    // the first flag is the default value
    pub fn flag_with_default(&mut self, flags: &[&str]) -> &mut Self {
        self.flags.push(Flag {
            values: flags.iter().map(|flag| flag.to_string()).collect(),
            optional: true,
            default: Some(0),
        });
        self
    }

    fn map_short_flag(&self, flag: &str) -> String {
        match self.short_flags.get(flag) {
            Some(f) => f.to_string(),
            None => flag.to_string(),
        }
    }

    pub fn short_flag(&mut self, flags: &[&str]) -> &mut Self {
        for flag in flags.iter() {
            let short_flag = flag.get(1..3).unwrap().to_string();

            if let Some(old) = self.short_flags.get(&short_flag) {
                panic!("{flag} and {old} have the same short name!")
            }

            self.short_flags.insert(short_flag, flag.to_string());
        }

        self
    }

    pub fn alias(&mut self, from: &str, to: &str) -> &mut Self {
        self.aliases.insert(from.to_string(), to.to_string());
        self
    }

    /// Let's say `raw_args` is `["rag", "ls-files", "--json", "--staged", "--name-only"]` and
    /// you don't care about the first 2 args (path and command name). You only want to parse
    /// the flags (the last 3 args). In this case, you set `skip_first_n` to 2.
    pub fn parse(&self, raw_args: &[String], skip_first_n: usize) -> Result<ParsedArgs, Error> {
        self.parse_worker(raw_args, skip_first_n)
            .map_err(|e| match e {
                Error::CliError(cli_err) => Error::CliError(cli_err),
                _ => e,
            })
    }

    fn parse_worker(&self, raw_args: &[String], skip_first_n: usize) -> Result<ParsedArgs, Error> {
        let mut args = vec![];
        let mut flags = vec![None; self.flags.len()];
        let mut arg_flags = HashMap::new();
        let mut expecting_flag_arg: Option<ArgFlag> = None;
        let mut no_more_flags = false;

        if raw_args.get(skip_first_n).map(|arg| arg.as_str()) == Some("--help") {
            return Ok(ParsedArgs {
                skip_first_n,
                raw_args: raw_args.to_vec(),
                args,
                flags: vec![],
                arg_flags,
                show_help: true,
            });
        }

        'raw_arg_loop: for (arg_index, raw_arg) in raw_args[skip_first_n..].iter().enumerate() {
            let raw_arg = match self.aliases.get(raw_arg) {
                Some(alias) => alias.to_string(),
                None => raw_arg.to_string(),
            };

            if raw_arg == "--" {
                if let Some(arg_flag) = expecting_flag_arg {
                    return Err(Error::CliError(CliError::MissingArgument(
                        arg_flag.flag.to_string(),
                        arg_flag.arg_type,
                        Span::End,
                    )));
                }

                no_more_flags = true;
                continue;
            }

            if let Some(arg_flag) = expecting_flag_arg {
                expecting_flag_arg = None;
                arg_flag
                    .arg_type
                    .parse(&raw_arg, Span::Exact(arg_index + skip_first_n))?;

                if let Some(_) = arg_flags.insert(arg_flag.flag.clone(), raw_arg.to_string()) {
                    return Err(Error::CliError(CliError::SameFlagMultipleTimes(
                        arg_flag.flag.clone(),
                        arg_flag.flag.clone(),
                        Span::Exact(arg_index + skip_first_n),
                    )));
                }

                continue;
            }

            if raw_arg.starts_with("-") && !no_more_flags {
                let mapped_flag = self.map_short_flag(&raw_arg);

                for (flag_index, flag) in self.flags.iter().enumerate() {
                    if flag.values.contains(&mapped_flag) {
                        if flags[flag_index].is_none() {
                            flags[flag_index] = Some(mapped_flag.to_string());
                            continue 'raw_arg_loop;
                        } else {
                            return Err(Error::CliError(CliError::SameFlagMultipleTimes(
                                flags[flag_index].as_ref().unwrap().to_string(),
                                raw_arg.to_string(),
                                Span::Exact(arg_index + skip_first_n),
                            )));
                        }
                    }
                }

                if let Some(arg_flag) = self.arg_flags.get(&mapped_flag) {
                    expecting_flag_arg = Some(arg_flag.clone());
                    continue;
                }

                if raw_arg.contains("=") {
                    let splitted = raw_arg.splitn(2, '=').collect::<Vec<_>>();
                    let flag = self.map_short_flag(splitted[0]);
                    let flag_arg = splitted[1];

                    if let Some(arg_flag) = self.arg_flags.get(&flag) {
                        arg_flag
                            .arg_type
                            .parse(flag_arg, Span::Exact(arg_index + skip_first_n))?;

                        if let Some(_) = arg_flags.insert(flag.to_string(), flag_arg.to_string()) {
                            return Err(Error::CliError(CliError::SameFlagMultipleTimes(
                                flag.to_string(),
                                flag.to_string(),
                                Span::Exact(arg_index + skip_first_n),
                            )));
                        }

                        continue;
                    } else {
                        return Err(Error::CliError(CliError::UnknownFlag {
                            flag: flag.to_string(),
                            similar_flag: self.get_similar_flag(&flag),
                            span: Span::Exact(arg_index + skip_first_n),
                        }));
                    }
                }

                return Err(Error::CliError(CliError::UnknownFlag {
                    flag: raw_arg.to_string(),
                    similar_flag: self.get_similar_flag(&raw_arg),
                    span: Span::Exact(arg_index + skip_first_n),
                }));
            } else {
                args.push(
                    self.arg_type
                        .parse(&raw_arg, Span::Exact(arg_index + skip_first_n))?,
                );
            }
        }

        if let Some(arg_flag) = expecting_flag_arg {
            return Err(Error::CliError(CliError::MissingArgument(
                arg_flag.flag.to_string(),
                arg_flag.arg_type,
                Span::End,
            )));
        }

        for i in 0..flags.len() {
            if flags[i].is_none() {
                if let Some(j) = self.flags[i].default {
                    flags[i] = Some(self.flags[i].values[j].clone());
                } else if !self.flags[i].optional {
                    return Err(Error::CliError(CliError::MissingFlag(
                        self.flags[i].values.join(" | "),
                        Span::End,
                    )));
                }
            }
        }

        loop {
            let span = match self.arg_count {
                ArgCount::Geq(n) if args.len() < n => Span::End,
                ArgCount::Leq(n) if args.len() > n => Span::NthArg(n + 1),
                ArgCount::Exact(n) if args.len() > n => Span::NthArg(n + 1),
                ArgCount::Exact(n) if args.len() < n => Span::NthArg(args.len().max(1) - 1),
                ArgCount::None if args.len() > 0 => Span::FirstArg,
                _ => {
                    break;
                }
            };

            return Err(Error::CliError(CliError::WrongArgCount {
                expected: self.arg_count,
                got: args.len(),
                span,
            }));
        }

        for (flag, arg_flag) in self.arg_flags.iter() {
            if arg_flags.contains_key(flag) {
                continue;
            } else if let Some(default) = &arg_flag.default {
                arg_flags.insert(flag.to_string(), default.to_string());
            } else if !arg_flag.optional {
                return Err(Error::CliError(CliError::MissingFlag(
                    flag.to_string(),
                    Span::End,
                )));
            }
        }

        Ok(ParsedArgs {
            skip_first_n,
            raw_args: raw_args.to_vec(),
            args,
            flags,
            arg_flags,
            show_help: false,
        })
    }

    fn get_similar_flag(&self, flag: &str) -> Option<String> {
        let mut candidates = vec![];

        for flag in self.flags.iter() {
            for flag in flag.values.iter() {
                candidates.push(flag.to_string());
            }
        }

        for flag in self.arg_flags.keys() {
            candidates.push(flag.to_string());
        }

        get_closest_string(&candidates, flag)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ArgCount {
    Geq(usize),
    Leq(usize),
    Exact(usize),
    Any,
    None,
}

/// `ArgType` doesn't do much. Only `Integer` and
/// `IntegerBetween` variants do extra checks. The
/// other variants are more like type signatures
/// which clarifies the intent of the code.
#[derive(Clone, Copy, Debug)]
pub enum ArgType {
    String,
    Path,
    Command,
    Query,
    UidOrPath,
    Integer,
    Url,

    /// Both inclusive
    IntegerBetween {
        min: Option<i128>,
        max: Option<i128>,
    },
}

impl ArgType {
    pub fn parse(&self, arg: &str, span: Span) -> Result<String, Error> {
        match self {
            ArgType::Integer => match arg.parse::<i128>() {
                Ok(_) => Ok(arg.to_string()),
                Err(e) => Err(Error::CliError(CliError::ParseIntError(e, span))),
            },
            ArgType::IntegerBetween { min, max } => match arg.parse::<i128>() {
                Ok(n) => {
                    if let Some(min) = *min {
                        if n < min {
                            return Err(Error::CliError(CliError::IntegerNotInRange {
                                min: Some(min),
                                max: *max,
                                n,
                                span,
                            }));
                        }
                    }

                    if let Some(max) = *max {
                        if n > max {
                            return Err(Error::CliError(CliError::IntegerNotInRange {
                                min: *min,
                                max: Some(max),
                                n,
                                span,
                            }));
                        }
                    }

                    Ok(arg.to_string())
                }
                Err(e) => Err(Error::CliError(CliError::ParseIntError(e, span))),
            },
            ArgType::String
            | ArgType::Path
            | ArgType::Url
            | ArgType::UidOrPath
            | ArgType::Command
            | ArgType::Query => Ok(arg.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flag {
    values: Vec<String>,
    optional: bool,
    default: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct ArgFlag {
    flag: String,
    optional: bool,
    default: Option<String>,
    arg_type: ArgType,
}

pub struct ParsedArgs {
    pub arg_flags: HashMap<String, String>,
    pub show_help: bool,
    pub raw_args: Vec<String>,
    pub skip_first_n: usize,
    pub args: Vec<String>,
    pub flags: Vec<Option<String>>,
}

impl ParsedArgs {
    pub fn new() -> Self {
        ParsedArgs {
            skip_first_n: 0,
            raw_args: vec![],
            args: vec![],
            flags: vec![],
            arg_flags: HashMap::new(),
            show_help: false,
        }
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_args_exact(&self, count: usize) -> Result<Vec<String>, Error> {
        if self.args.len() == count {
            Ok(self.args.clone())
        } else {
            Err(Error::CliError(CliError::WrongArgCount {
                expected: ArgCount::Exact(count),
                got: self.args.len(),
                span: Span::FirstArg.render(&self.raw_args, self.skip_first_n),
            }))
        }
    }

    // if there's an index error, it panics instead of returning None
    // if it returns None, that means Nth flag is optional and its value is None
    pub fn get_flag(&self, index: usize) -> Option<String> {
        self.flags[index].clone()
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }
}

/// It parses `rag [-C <path>] <command> <args>` and returns
/// `Ok((args, pre_args))` where `args` is `rag <command> <args>` and
/// `pre_args` is `-C <path>`.
///
/// NOTE: Do not use this function. I have implemented this because I'm not sure
/// how to implement `-C` option. I'll remove this function as soon as I come up
/// with a nice way to implement `-C`.
///
/// It only supports `-C <path>` and not `-C=<path>` and that's intentional. Git
/// neither supports `-C=<path>` (I don't know why), and I decided to blindly follow that.
pub fn parse_pre_args(args: &[String]) -> Result<(Vec<String>, ParsedArgs), Error> {
    match args.get(1).map(|s| s.as_str()) {
        Some("-C") => match args.get(2).map(|s| s.as_str()) {
            Some(path) => {
                let mut result = ParsedArgs::new();
                result
                    .arg_flags
                    .insert(String::from("-C"), path.to_string());
                Ok((
                    vec![
                        vec![args[0].clone()],
                        if args.len() < 4 {
                            vec![]
                        } else {
                            args[3..].to_vec()
                        },
                    ]
                    .concat(),
                    result,
                ))
            }
            None => Err(Error::CliError(CliError::MissingArgument(
                String::from("-C"),
                ArgType::Path,
                Span::Exact(2),
            ))),
        },
        _ => Ok((args.to_vec(), ParsedArgs::new())),
    }
}

impl std::fmt::Display for ArgCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgCount::Geq(n) => write!(f, "at least {}", n),
            ArgCount::Leq(n) => write!(f, "at most {}", n),
            ArgCount::Exact(n) => write!(f, "exactly {}", n),
            ArgCount::Any => write!(f, "any"),
            ArgCount::None => write!(f, "none"),
        }
    }
}
