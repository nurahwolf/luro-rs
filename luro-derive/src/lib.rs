use proc_macro::TokenStream;

mod functions;
mod slash_command;

#[proc_macro_attribute]
pub fn slash_command(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = match darling::ast::NestedMeta::parse_meta_list(args.into()) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error().into(),
    };

    let args = match <slash_command::SlashCommandArgs as darling::FromMeta>::from_list(&args) {
        Ok(x) => x,
        Err(e) => return e.write_errors().into(),
    };

    let function = syn::parse_macro_input!(function as syn::ItemFn);

    match slash_command::slash_command(args, function) {
        Ok(slash_command) => slash_command,
        Err(e) => e.write_errors().into(),
    }
}
