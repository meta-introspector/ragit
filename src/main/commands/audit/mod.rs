use crate::prelude::*;
use ragit_api::AuditRecord as Audit;
use crate::LoadMode;
use crate::main::commands::audit_command::audit_command;

mod args;
mod output;

use args::AuditArgs;
use output::print_audit_results;

pub async fn audit_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    let audit_args = AuditArgs::parse(&args)?;

    let index = Index::load(crate::main::find_root()?, LoadMode::Minimum)?;
    let mut result = index.audit(if audit_args.this_week { Some(audit_args.since) } else { None })?;
    let mut total = Audit::default();

    for a in result.values() {
        total += *a;
    }

    result.insert(String::from("total"), total);

    print_audit_results(&audit_args, &result)?;

    Ok(())
}
