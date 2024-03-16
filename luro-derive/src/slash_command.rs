use syn::spanned::Spanned;

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub struct SlashCommandArgs {
    nsfw: bool,
}

use darling::Error;
use proc_macro::TokenStream;

use crate::functions::extract_help_from_doc_comments;

pub fn slash_command(args: SlashCommandArgs, function: syn::ItemFn) -> Result<TokenStream, Error> {
    // Ensure that the function is async
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new(function.sig.span(), "command function must be async").into());
    }

    // The function should have an output of `Result<(), ...>`
    if function.sig.output == syn::ReturnType::Default {
        return Err(syn::Error::new(function.sig.span(), "command function must return Result<(), ...>").into());
    }

    // Extract the command descriptions from the function doc comments
    let (description, help_text) = extract_help_from_doc_comments(&function.attrs);

    // Set variables needed for crafting the command
    let _function_name = function.sig.ident.to_string().trim_start_matches("r#").to_string();
    let nsfw = args.nsfw;
    let description = match &description {
        Some(x) => quote::quote! { Some(#x.to_string()) },
        None => quote::quote! { None },
    };

    let _function_visibility = &function.vis;

    // let slash_command = quote::quote! {
    //     #function_visibility fn #function_ident #function_generics() -> ::luro_derive::SlashCommand<
    //     > {
    //         #function

    //         ::luro_derive::SlashCommand {
    //             command: #command,
    //             name: #function_name,
    //             description: #description,
    //             long_description: #help_text,
    //             nsfw: #nsfw,
    //         }
    //     }
    // };

    let command_name = &function.sig.ident.to_string();
    let struct_path = quote::quote!(::luro_model::command::SlashCommand);

    let slash_command = quote::quote! {
        pub static #command_name: #struct_path = #struct_path {
            command: (),
            name: #command_name,
            description: #description,
            long_description: #help_text,
            nsfw: #nsfw,
        }
    };

    println!("{slash_command:#?}");

    Ok(slash_command.into())
}
