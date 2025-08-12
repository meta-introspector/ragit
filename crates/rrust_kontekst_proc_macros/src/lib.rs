#[proc_macro_attribute]
pub fn mcp_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    let mut config = match parse_macro_args_helper(args) {
        Ok(config) => config,
        Err(err) => return err.to_compile_error().into(),
    };
    
    if config.label.is_empty() {
        config.label = fn_name_str.clone();
    }
    if config.tool_name.is_empty() {
        config.tool_name = fn_name_str.to_lowercase().replace("component", "");
    }
    
    let handler_function = generate_handler_function(&config, fn_name);
    let registration = generate_registration(&config, fn_name);
    let metadata = generate_metadata(&config, fn_name, &input_fn);
    
    let expanded = quote! {
        #input_fn
        
        #handler_function
        
        #registration
        
        #metadata
    };

    TokenStream::from(expanded)
}