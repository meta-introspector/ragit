use std::fs;
use syn::{parse_file, visit_mut::VisitMut, visit_mut, ItemFn, UseTree, UsePath, UseGlob, UseName, UseRename};
use quote::quote;

struct RefactorVisitor;

impl VisitMut for RefactorVisitor {
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        // Change `pub async fn run_quiz()` to `pub fn run_quiz()`
        if i.sig.ident.to_string() == "run_quiz" {
            if i.sig.asyncness.is_some() {
                i.sig.asyncness = None;
                // Remove .await from the call if present
                // This is a simplified approach, a full AST transformation would be more robust
                let new_block = quote! {
                    #i.block
                };
                i.block = syn::parse2(new_block).unwrap();
            }
        }
        visit_mut::visit_item_fn_mut(self, i);
    }

    fn visit_use_tree_mut(&mut self, i: &mut UseTree) {
        // Comment out `use hf_dataset_validator::hf_dataset_converter;` and `use tokio;`
        if let UseTree::Path(UsePath { ident, tree, .. }) = i {
            if ident.to_string() == "hf_dataset_validator" || ident.to_string() == "tokio" {
                *i = UseTree::Verbatim(quote! { /* use #ident #tree ; */ });
            }
        }
        visit_mut::visit_use_tree_mut(self, i);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Process quiz_logic.rs
    let quiz_logic_path = "crates/term_quiz_master/src/quiz_logic.rs";
    let mut quiz_logic_content = fs::read_to_string(quiz_logic_path)?;
    let mut quiz_logic_ast = parse_file(&quiz_logic_content)?;

    let mut visitor = RefactorVisitor;
    visitor.visit_file_mut(&mut quiz_logic_ast);

    quiz_logic_content = quote! { #quiz_logic_ast }.to_string();

    // Comment out the generate_parquet block
    let start_marker = "    if args.generate_parquet {";
    let end_marker = "    }";
    if let Some(start) = quiz_logic_content.find(start_marker) {
        if let Some(end) = quiz_logic_content[start..].find(end_marker) {
            let end = start + end + end_marker.len();
            let block = &quiz_logic_content[start..end];
            let commented_block = block.lines().map(|line| format!("// {{}}", line)).collect::<Vec<String>>().join("\n");
            quiz_logic_content.replace_range(start..end, &commented_block);
        }
    }
    fs::write(quiz_logic_path, quiz_logic_content)?;

    // Process main.rs
    let main_path = "crates/term_quiz_master/src/main.rs";
    let mut main_content = fs::read_to_string(main_path)?;
    let mut main_ast = parse_file(&main_content)?;

    // Remove #[tokio::main] and .await
    let new_main_content = main_content
        .replace("#[tokio::main]", "")
        .replace(".await", "");
    fs::write(main_path, new_main_content)?;

    Ok(())
}