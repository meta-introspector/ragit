use ragit_utils::prelude::*;
use ragit_api::AuditRecord as Audit;
use ragit_types::prelude::*;
use std::collections::HashMap;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::Index;
//use ragit_index_types::Index;


use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_utils::cli_types::CliError;
use chrono::{Local, Days};
use serde_json::Value;
use ragit_index_core::load_index_from_path;
pub async fn audit_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), anyhow::Error> {
    let audit_args = AuditArgs::parse(&args)?;

    let index = load_index_from_path(&find_root()?)?;
    let mut result = index.audit(if audit_args.this_week {
        Some(audit_args.since)
    } else {
        None
    })?;
    let mut total = Audit::default();

    for a in result.values() {
        total += *a;
    }

    result.insert(String::from("total"), total);

    print_audit_results(&audit_args, &result)?;

    Ok(())
}

pub struct AuditArgs {
    pub this_week: bool,
    pub since: chrono::DateTime<chrono::Local>,
    pub category: Option<String>,
    pub show_tokens: bool,
    pub show_costs: bool,
    pub json_mode: bool,
}

impl AuditArgs {
    pub fn parse(args: &[String]) -> Result<Self, anyhow::Error> {
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

pub fn print_audit_results(args: &AuditArgs, result: &HashMap<String, Audit>) -> Result<(), anyhow::Error> {
    if let Some(category) = &args.category {
        print_single_category(args, result, category)?;
    } else {
        print_all_categories(args, result)?;
    }
    Ok(())
}

fn print_single_category(
    args: &AuditArgs,
    result: &HashMap<String, Audit>,
    category: &str,
) -> Result<(), anyhow::Error> {
    let audit = match result.get(category) {
        Some(r) => *r,
        None => {
            return Err(anyhow::anyhow!(CliError::new_message(
                format!("`{category}` is an invalid category."),
            )));
        }
    };

    if args.json_mode {
        println!(
            "{{ \"category\": {category:?}{}{} }}",
            if args.show_tokens {
                format!(
                    ", \"total tokens\": {}, \"input tokens\": {}, \"output tokens\": {}",
                    audit.input_tokens + audit.output_tokens,
                    audit.input_tokens,
                    audit.output_tokens
                )
            } else {
                String::new()
            },
            if args.show_tokens && args.show_costs {
                ", "
            } else {
                ""
            if args.show_costs {
                format!(
                    ", \"total cost\": {:.03}, \"input cost\": {:.03}, \"output cost\": {:.03}",
                    (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0,
                    audit.input_cost as f64 / 1_000_000.0,
                    audit.output_cost as f64 / 1_000_000.0
                )
            } else {
                String::new()
            }
        );
    } else {
        println!("category: {category}");

        if args.show_tokens {
            println!(
                "    total tokens:  {}",
                audit.input_tokens + audit.output_tokens
            );
            println!("    input tokens:  {}", audit.input_tokens);
            println!("    output tokens: {}", audit.output_tokens);
        }

        if args.show_costs {
            println!(
                "    total cost:  {:.03}$",
                (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0
            );
            println!(
                "    input cost:  {:.03}$",
                    audit.input_cost as f64 / 1_000_000.0
                );
                println!(
                    "    output cost: {:.03}$",
                    audit.output_cost as f64 / 1_000_000.0
                );
            }
        }
        Ok(())
}

fn print_all_categories(args: &AuditArgs, result: &HashMap<String, Audit>) -> Result<(), anyhow::Error> {
    let mut sorted_categories = result
        .keys()
        .map(|category| category.to_string())
        .collect::<Vec<_>>();
    sorted_categories.sort();
    sorted_categories = sorted_categories
        .into_iter()
        .filter(|category| category != "total")
        .collect();
    sorted_categories.insert(0, String::from("total"));

    if args.json_mode {
        let mut map = serde_json::Map::<String, Value>::new();

        for category in sorted_categories.iter() {
            let mut entry = serde_json::Map::<String, Value>::new();
            let audit = result.get(category).unwrap();

            if args.show_tokens {
                entry.insert(
                    String::from("total tokens"),
                    (audit.input_tokens + audit.output_tokens).into(),
                );
                entry.insert(String::from("input tokens"), audit.input_tokens.into());
                entry.insert(String::from("output tokens"), audit.output_tokens.into());
            }

            if args.show_costs {
                entry.insert(
                    String::from("total cost"),
                    ((audit.input_cost + audit.output_cost) as f64 / 1_000_000.0).into(),
                );
                entry.insert(
                    String::from("input cost"),
                    (audit.input_cost as f64 / 1_000_000.0).into(),
                );
                entry.insert(
                    String::from("output cost"),
                    (audit.output_cost as f64 / 1_000_000.0).into(),
                );
            }

            map.insert(category.to_string(), entry.into());
        }

        println!("{}", serde_json::to_string_pretty(&map)?);
    } else {
        for category in sorted_categories.iter() {
            let audit = result.get(category).unwrap();
            println!("category: {category}");

            if args.show_tokens {
                println!(
                    "    total tokens:  {}",
                    audit.input_tokens + audit.output_tokens
                );
                println!("    input tokens:  {}", audit.input_tokens);
                println!("    output tokens: {}", audit.output_tokens);
            }

            if args.show_costs {
                println!(
                    "    total cost:  {:.03}$",
                    (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0
                );
                println!(
                    "    input cost:  {:.03}$",
                    audit.input_cost as f64 / 1_000_000.0
                );
                println!(
                    "    output cost: {:.03}$",
                    audit.output_cost as f64 / 1_000_000.0
                );
            }
        }
    }
    Ok(())
}
