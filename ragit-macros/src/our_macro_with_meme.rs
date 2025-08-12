use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(OurMacroWithMeme, attributes(meme))]
pub fn our_macro_with_meme_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let meme_attr = ast.attrs.iter().find(|a| a.path.is_ident("meme"));
    let meme_literal = if let Some(attr) = meme_attr {
        if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
            if let Lit::Str(lit) = meta.lit {
                lit.value()
            } else {
                "_placeholder_".to_string()
            }
        } else {
            "_placeholder_".to_string()
        }
    } else {
        "_placeholder_".to_string()
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn get_meme(&self) -> &'static str {
                #meme_literal
            }
        }
    };

    expanded.into()
}
