use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn instrument_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let function_name = &input.sig.ident;
    let function_body = &input.block;
    let function_signature = &input.sig;
    let visibility = &input.vis;
    let attributes = &input.attrs;

    let expanded = quote! {
        #(#attributes)*
        #visibility #function_signature {
            println!("Entering function: {}", stringify!(#function_name));
            let result = #function_body;
            println!("Exiting function: {}", stringify!(#function_name));
            result
        }
    };
    TokenStream::from(expanded)
}