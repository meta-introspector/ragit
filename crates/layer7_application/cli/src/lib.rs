//use std::collections::HashMap;

pub use ragit_utils::error::Error;

pub use ragit_utils::cli_types::{ArgCount, ArgParser, ArgType, ParsedArgs, Span};
//use ragit_utils::error::ErrorKind;

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
            None => Err(Error::CliError(
                ragit_utils::error::CliError::MissingArgument(
                    String::from("-C"),
                    ArgType::Path,
                    Span::Exact(2),
                ),
            )),
        },
        _ => Ok((args.to_vec(), ParsedArgs::new())),
    }
}
