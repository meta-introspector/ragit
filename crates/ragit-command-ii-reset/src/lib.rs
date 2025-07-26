use ragit_utils::prelude::*;
 use ragit_api::prelude::*;
 use ragit_types::prelude::*;
 use ragit_index_types::index_struct::Index;
 use ragit_index_types::index_struct::Index;
 use ragit_index_core::load_index_from_path;
 
 use ragit_utils::project_root::find_root;
 use ragit_utils::doc_utils::get_doc_content;
 
 pub async fn ii_reset_command_main(args: &[String]) -> Result<(), anyhow::Error> {
     let parsed_args = ArgParser::new()
         .optional_flag(&["--hard"])
         .args(ArgType::UidOrPath, ArgCount::Any)
         .parse(args, 2)?;
 
     if parsed_args.show_help() {
         println!("{}", get_doc_content("commands/ii-reset.txt"));
         return Ok(());
     }
 
     let mut index = load_index_from_path(&find_root()?)?;
     let hard = parsed_args.get_flag(0).is_some();
 
     index.reset_ii()?;
 
     Ok(())
 }
