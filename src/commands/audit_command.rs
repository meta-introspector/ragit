use chrono::{Days, Local};
use ragit::{Audit, Error, Index, LoadMode, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};
use serde_json::Value;

pub async fn audit_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--this-week"])
        .optional_flag(&["--only-tokens", "--only-costs"])
        .optional_arg_flag("--category", ArgType::String)  // NOTE: there's no `ArgType` for audit categories
        .optional_flag(&["--json"])
        .short_flag(&["--category", "--json"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/audit.txt"));
        return Ok(());
    }

    let this_week = parsed_args.get_flag(0).is_some();
    let since = Local::now().checked_sub_days(Days::new(7)).unwrap();
    let category = parsed_args.arg_flags.get("--category").map(|c| c.to_string());
    let show_tokens = parsed_args.get_flag(1).unwrap_or_else(|| String::from("--only-tokens")) != "--only-costs";
    let show_costs = parsed_args.get_flag(1).unwrap_or_else(|| String::from("--only-costs")) != "--only-tokens";
    let json_mode = parsed_args.get_flag(2).is_some();
    let index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::Minimum)?;
    let mut result = index.audit(if this_week { Some(since) } else { None })?;
    let mut total = Audit::default();

    for a in result.values() {
        total += *a;
    }

    result.insert(String::from("total"), total);

    if let Some(category) = &category {
        let result = match result.get(category) {
            Some(r) => *r,
            None => {
                return Err(Error::CliError {
                    message: format!("`{category}` is an invalid category."),
                    span: (String::new(), 0, 0),  // TODO
                });
            },
        };

        if json_mode {
            println!(
                "{{ \"category\": {category:?}, {}{}{} }}",
                if show_tokens { format!("\"total tokens\": {}, \"input tokens\": {}, \"output tokens\": {}", result.input_tokens + result.output_tokens, result.input_tokens, result.output_tokens) } else { String::new() },
                if show_tokens && show_costs { ", " } else { "" },
                if show_costs { format!("\"total cost\": {:.03}, \"input cost\": {:.03}, \"output cost\": {:.03}", (result.input_cost + result.output_cost) as f64 / 1_000_000.0, result.input_cost as f64 / 1_000_000.0, result.output_cost as f64 / 1_000_000.0) } else { String::new() },
            );
        }

        else {
            println!("category: {category}");

            if show_tokens {
                println!("    total tokens:  {}", result.input_tokens + result.output_tokens);
                println!("    input tokens:  {}", result.input_tokens);
                println!("    output tokens: {}", result.output_tokens);
            }

            if show_costs {
                println!("    total cost:  {:.03}$", (result.input_cost + result.output_cost) as f64 / 1_000_000.0);
                println!("    input cost:  {:.03}$", result.input_cost as f64 / 1_000_000.0);
                println!("    output cost: {:.03}$", result.output_cost as f64 / 1_000_000.0);
            }
        }
    }

    else {
        // for readability, it sorts the keys
        let mut sorted_categories = result.keys().map(|category| category.to_string()).collect::<Vec<_>>();
        sorted_categories.sort();
        sorted_categories = sorted_categories.into_iter().filter(|category| category != "total").collect();
        sorted_categories.insert(0, String::from("total"));

        if json_mode {
            let mut map = serde_json::Map::new();

            for category in sorted_categories.iter() {
                let mut entry = serde_json::Map::new();
                let audit = result.get(category).unwrap();

                if show_tokens {
                    entry.insert(String::from("total tokens"), (audit.input_tokens + audit.output_tokens).into());
                    entry.insert(String::from("input tokens"), audit.input_tokens.into());
                    entry.insert(String::from("output tokens"), audit.output_tokens.into());
                }

                if show_costs {
                    entry.insert(String::from("total cost"), ((audit.input_cost + audit.output_cost) as f64 / 1_000_000.0).into());
                    entry.insert(String::from("input cost"), (audit.input_cost as f64 / 1_000_000.0).into());
                    entry.insert(String::from("output cost"), (audit.output_cost as f64 / 1_000_000.0).into());
                }

                map.insert(category.to_string(), entry.into());
            }

            println!("{}", serde_json::to_string_pretty(&map)?);
        }

        else {
            for category in sorted_categories.iter() {
                let audit = result.get(category).unwrap();
                println!("category: {category}");

                if show_tokens {
                    println!("    total tokens:  {}", audit.input_tokens + audit.output_tokens);
                    println!("    input tokens:  {}", audit.input_tokens);
                    println!("    output tokens: {}", audit.output_tokens);
                }

                if show_costs {
                    println!("    total cost:  {:.03}$", (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0);
                    println!("    input cost:  {:.03}$", audit.input_cost as f64 / 1_000_000.0);
                    println!("    output cost: {:.03}$", audit.output_cost as f64 / 1_000_000.0);
                }
            }
        }
    }
    Ok(())
}