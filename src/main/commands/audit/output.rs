use crate::prelude::*;
use std::collections::HashMap;
use ragit_api::AuditRecord as Audit;

use super::args::AuditArgs;

pub fn print_audit_results(
    args: &AuditArgs,
    result: &HashMap<String, Audit>,
) -> Result<(), Error> {
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
) -> Result<(), Error> {
    let audit = match result.get(category) {
        Some(r) => *r,
        None => {
            return Err(Error::CliError {
                message: format!("`{category}` is an invalid category."),
                span: (String::new(), 0, 0), // TODO
            });
        }
    };

    if args.json_mode {
        println!(
            "{{ \"category\": {category:?}, {}{}{} }}",
            if args.show_tokens { format!("\"total tokens\": {}, \"input tokens\": {}, \"output tokens\": {}", audit.input_tokens + audit.output_tokens, audit.input_tokens, audit.output_tokens) } else { String::new() },
            if args.show_tokens && args.show_costs { ", " } else { "" },
            if args.show_costs { format!("\"total cost\": {:.03}, \"input cost\": {:.03}, \"output cost\": {:.03}", (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0, audit.input_cost as f64 / 1_000_000.0, audit.output_cost as f64 / 1_000_000.0) } else { String::new() },
        );
    } else {
        println!("category: {category}");

        if args.show_tokens {
            println!("    total tokens:  {}", audit.input_tokens + audit.output_tokens);
            println!("    input tokens:  {}", audit.input_tokens);
            println!("    output tokens: {}", audit.output_tokens);
        }

        if args.show_costs {
            println!("    total cost:  {:.03}$", (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0);
            println!("    input cost:  {:.03}$", audit.input_cost as f64 / 1_000_000.0);
            println!("    output cost: {:.03}$", audit.output_cost as f64 / 1_000_000.0);
        }
    }
    Ok(())
}

fn print_all_categories(
    args: &AuditArgs,
    result: &HashMap<String, Audit>,
) -> Result<(), Error> {
    let mut sorted_categories = result.keys().map(|category| category.to_string()).collect::<Vec<_>>();
    sorted_categories.sort();
    sorted_categories = sorted_categories.into_iter().filter(|category| category != "total").collect();
    sorted_categories.insert(0, String::from("total"));

    if args.json_mode {
        let mut map = serde_json::Map::new();

        for category in sorted_categories.iter() {
            let mut entry = serde_json::Map::new();
            let audit = result.get(category).unwrap();

            if args.show_tokens {
                entry.insert(String::from("total tokens"), (audit.input_tokens + audit.output_tokens).into());
                entry.insert(String::from("input tokens"), audit.input_tokens.into());
                entry.insert(String::from("output tokens"), audit.output_tokens.into());
            }

            if args.show_costs {
                entry.insert(String::from("total cost"), ((audit.input_cost + audit.output_cost) as f64 / 1_000_000.0).into());
                entry.insert(String::from("input cost"), (audit.input_cost as f64 / 1_000_000.0).into());
                entry.insert(String::from("output cost"), (audit.output_cost as f64 / 1_000_000.0).into());
            }

            map.insert(category.to_string(), entry.into());
        }

        println!("{}", serde_json::to_string_pretty(&map)?);
    } else {
        for category in sorted_categories.iter() {
            let audit = result.get(category).unwrap();
            println!("category: {category}");

            if args.show_tokens {
                println!("    total tokens:  {}", audit.input_tokens + audit.output_tokens);
                println!("    input tokens:  {}", audit.input_tokens);
                println!("    output tokens: {}", audit.output_tokens);
            }

            if args.show_costs {
                println!("    total cost:  {:.03}$", (audit.input_cost + audit.output_cost) as f64 / 1_000_000.0);
                println!("    input cost:  {:.03}$", audit.input_cost as f64 / 1_000_000.0);
                println!("    output cost: {:.03}$", audit.output_cost as f64 / 1_000_000.0);
            }
        }
    }
    Ok(())
}
