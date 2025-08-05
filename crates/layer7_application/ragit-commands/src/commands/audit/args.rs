use crate::prelude::*;
use chrono::{Local, Days};

pub struct AuditArgs {
    pub this_week: bool,
    pub since: chrono::DateTime<chrono::Local>,
    pub category: Option<String>,
    pub show_tokens: bool,
    pub show_costs: bool,
    pub json_mode: bool,
}

impl AuditArgs {
    pub fn parse(args: &[String]) -> Result<Self, Error> {
        let parsed_args = ArgParser::new()
            .optional_flag(&["--this-week"])
            .optional_flag(&["--only-tokens", "--only-costs"])
            .optional_arg_flag("--category", ArgType::String)
            .optional_flag(&["--json"])
            .short_flag(&["--category", "--json"])
            .parse(args, 2)?;

        if parsed_args.show_help() {
            println!("{}", get_doc_content("commands/audit.txt"));
            std::process::exit(0);
        }

        Ok(Self {
            this_week: parsed_args.get_flag(0).is_some(),
            since: Local::now().checked_sub_days(Days::new(7)).unwrap(),
            category: parsed_args
                .arg_flags
                .get("--category")
                .map(|c| c.to_string()),
            show_tokens: parsed_args
                .get_flag(1)
                .unwrap_or_else(|| String::from("--only-tokens"))
                != "--only-costs",
            show_costs: parsed_args
                .get_flag(1)
                .unwrap_or_else(|| String::from("--only-costs"))
                != "--only-tokens",
            json_mode: parsed_args.get_flag(2).is_some(),
        })
    }
}
