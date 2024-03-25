use syn::spanned::Spanned;

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub struct SlashCommandArgs {
    nsfw: bool,
    ephemeral: bool,
}

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
struct SlashCommandParameterArgs {
    description: Option<String>,
    rename: Option<String>,
    flag: bool,
}

use darling::Error;
use proc_macro::TokenStream;

use crate::functions::{extract_help_from_doc_comments, extract_type_parameter};

pub fn slash_command(args: SlashCommandArgs, mut function: syn::ItemFn) -> Result<TokenStream, Error> {
    // Ensure that the function is async
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new(function.sig.span(), "command function must be async").into());
    }

    // The function should have an output of `Result<(), ...>`
    if function.sig.output == syn::ReturnType::Default {
        return Err(syn::Error::new(function.sig.span(), "command function must return Result<(), ...>").into());
    }

    // Extract the command descriptions from the function doc comments
    let (description, help_text) = match extract_help_from_doc_comments(&function.attrs) {
        (None, help_text) => ("No Description Provided".to_owned(), help_text),
        (Some(description), help_text) => (description, help_text),
    };

    // Handle the parameters present
    let command_parameters = command_parameters(&mut function)?;

    // Details needed for the function itself
    let command_name = &function.sig.ident.to_string();
    let function_ident = std::mem::replace(&mut function.sig.ident, syn::parse_quote! { inner });
    let function_generics = &function.sig.generics;
    let function_visibility = &function.vis;
    let function = &function;

    // Details needed for the slash command
    let nsfw = args.nsfw;
    let ephemeral = args.ephemeral;

    let slash_command = quote::quote! {
        #function_visibility fn #function_ident -> ::luro_model::command::SlashCommand {
            #function

            ::luro_model::command::SlashCommand {
                command: Box::pin(async move {}),
                name: #command_name.to_string(),
                description: #description.to_string(),
                long_description: Some("WTF".to_owned()),
                parameters: vec![ #( #command_parameters ),* ],
                nsfw: #nsfw,
                ephemeral: #ephemeral
            }
        }
    };

    Ok(slash_command.into())
}

fn command_parameters(function: &mut syn::ItemFn) -> Result<Vec<proc_macro2::TokenStream>, Error> {
    let mut command_parameters = Vec::new();
    for command_parameter in function.sig.inputs.iter_mut().skip(1) {
        let pattern = match command_parameter {
            syn::FnArg::Typed(x) => x,
            syn::FnArg::Receiver(r) => {
                return Err(syn::Error::new(r.span(), "self argument is invalid here").into());
            }
        };

        let attrs: Vec<_> = pattern
            .attrs
            .drain(..)
            .map(|attr| darling::ast::NestedMeta::Meta(attr.meta))
            .collect();
        let arguments = <SlashCommandParameterArgs as darling::FromMeta>::from_list(&attrs)?;

        let command_option_name = if let Some(rename) = &arguments.rename {
            rename.clone()
        } else if let syn::Pat::Ident(ident) = &*pattern.pat {
            ident.ident.to_string().trim_start_matches("r#").into()
        } else {
            let message = "#[rename = \"...\"] must be specified for pattern parameters";
            return Err(syn::Error::new(pattern.pat.span(), message).into());
        };

        // no #[description] check here even if slash_command set, so users can programatically
        // supply descriptions later (e.g. via translation framework like fluent)
        let description = match arguments.description {
            Some(ref description) => description.clone(),
            None => "No description for this paramater has been set.".to_owned(),
        };

        let (mut required, _type_) =
            match extract_type_parameter("Option", &pattern.ty).or_else(|| extract_type_parameter("Vec", &pattern.ty)) {
                Some(t) => (false, t),
                None => (true, pattern.ty.as_ref()),
            };

        // Don't require user to input a value for flags - use false as default value (see below)
        if arguments.flag {
            required = false;
        }

        // // We can just cast to f64 here because Discord only uses f64 precision anyways
        // // TODO: move this to poise::CommandParameter::{min, max} fields
        // let min_value_setter = match &param.args.min {
        //     Some(x) => quote::quote! { .min_number_value(#x as f64) },
        //     None => quote::quote! {},
        // };
        // let max_value_setter = match &param.args.max {
        //     Some(x) => quote::quote! { .max_number_value(#x as f64) },
        //     None => quote::quote! {},
        // };
        // // TODO: move this to poise::CommandParameter::{min_length, max_length} fields
        // let min_length_setter = match &param.args.min_length {
        //     Some(x) => quote::quote! { .min_length(#x) },
        //     None => quote::quote! {},
        // };
        // let max_length_setter = match &param.args.max_length {
        //     Some(x) => quote::quote! { .max_length(#x) },
        //     None => quote::quote! {},
        // };

        // let type_setter = match inv.args.slash_command {
        //     true => {
        //         if let Some(_choices) = &param.args.choices {
        //             quote::quote! { Some(|o| o.kind(::poise::serenity_prelude::CommandOptionType::Integer)) }
        //         } else {
        //             quote::quote! { Some(|o| {
        //                 poise::create_slash_argument!(#type_, o)
        //                 #min_value_setter #max_value_setter
        //                 #min_length_setter #max_length_setter
        //             }) }
        //         }
        //     }
        //     false => quote::quote! { None },
        // };

        // // TODO: theoretically a problem that we don't store choices for non slash commands
        // // TODO: move this to poise::CommandParameter::choices (is there a reason not to?)
        // let choices = match inv.args.slash_command {
        //     true => {
        //         if let Some(choices) = &param.args.choices {
        //             let choices = &choices.0;
        //             quote::quote! { vec![#( ::poise::CommandParameterChoice {
        //                 name: ToString::to_string(&#choices),
        //                 localizations: Default::default(),
        //                 __non_exhaustive: (),
        //             } ),*] }
        //         } else {
        //             quote::quote! { poise::slash_argument_choices!(#type_) }
        //         }
        //     }
        //     false => quote::quote! { vec![] },
        // };

        // let channel_types = match &param.args.channel_types {
        //     Some(crate::util::List(channel_types)) => quote::quote! { Some(
        //         vec![ #( poise::serenity_prelude::ChannelType::#channel_types ),* ]
        //     ) },
        //     None => quote::quote! { None },
        // };

        command_parameters.push((
            quote::quote! {
                ::luro_model::command::SlashCommandParameter {
                    name: #command_option_name,
                    description: #description,
                    required: #required,
                }
            },
            required,
        ));
    }

    // Sort the commands so that optional (non-required) are last. Then drop that information from the vec.
    command_parameters.sort_by_key(|(_, required)| !required);
    let command_parameters = command_parameters
        .into_iter()
        .map(|(command_parameters, _)| command_parameters)
        .collect();
    Ok(command_parameters)
}
