use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, DataEnum, DeriveInput, Error, Ident, Result};

use super::parse::{ChoiceKind, ChoiceValue, ParsedVariant};

/// Implementation of the `CommandOption` derive macro
pub fn impl_command_option(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let input_span = input.span();

    let (variants, kind) = match input.data {
        syn::Data::Enum(DataEnum { variants, .. }) => ParsedVariant::from_variants(variants, input_span)?,
        _ => {
            return Err(Error::new(
                input_span,
                "`#[derive(CommandOption)] can only be applied to enums"
            ))
        }
    };

    let parsed_init = parsed_init(kind);
    let match_expr = match_expr(kind);
    let match_arms = variants.iter().map(variant_match_arm);

    let value_match_arms = variants.iter().map(value_match_arm);
    let choice_ty = match kind {
        ChoiceKind::String => quote! { &'static str },
        ChoiceKind::Integer => quote! { i64 },
        ChoiceKind::Number => quote! { f64 }
    };

    Ok(quote! {
        impl ::twilight_interactions::command::CommandOption for #ident {
            fn from_option(
                value: ::twilight_model::application::interaction::application_command::CommandOptionValue,
                data: twilight_interactions::command::internal::CommandOptionData,
                resolved: ::std::option::Option<&::twilight_model::application::interaction::InteractionDataResolved>
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseOptionErrorType> {
                #parsed_init

                match #match_expr {
                    #(#match_arms,)*
                    other => ::std::result::Result::Err(
                        ::twilight_interactions::error::ParseOptionErrorType::InvalidChoice(
                            ::std::string::ToString::to_string(other)
                        )
                    )
                }
            }
        }

        impl #ident {
            /// Get the value corresponding to the current variant.
            ///
            /// This method is automatically generated by the [`CommandOption`] derive macro.
            ///
            /// [`CommandOption`]: twilight_interactions::command::CommandOption
            pub fn value(&self) -> #choice_ty {
                match self {
                    #(#value_match_arms,)*
                }
            }
        }
    })
}

/// Dummy implementation of the `CommandOption` trait in case of macro error
pub fn dummy_command_option(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::command::CommandOption for #ident {
            fn from_option(
                value: ::twilight_model::application::interaction::application_command::CommandOptionValue,
                data: ::twilight_interactions::command::internal::CommandOptionData,
                resolved: ::std::option::Option<&::twilight_model::application::interaction::InteractionDataResolved>
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseOptionErrorType> {
                ::std::unimplemented!()
            }
        }
    }
}

/// Generate parsed variable initialization
fn parsed_init(kind: ChoiceKind) -> TokenStream {
    match kind {
        ChoiceKind::String => {
            quote! { let parsed: ::std::string::String = ::twilight_interactions::command::CommandOption::from_option(value, ::std::default::Default::default(), resolved)?; }
        }
        ChoiceKind::Integer => {
            quote! { let parsed: i64 = ::twilight_interactions::command::CommandOption::from_option(value, ::std::default::Default::default(), resolved)?; }
        }
        ChoiceKind::Number => {
            quote! { let parsed: f64 = ::twilight_interactions::command::CommandOption::from_option(value, ::std::default::Default::default(), resolved)?; }
        }
    }
}

/// Expression in match block
fn match_expr(kind: ChoiceKind) -> TokenStream {
    match kind {
        ChoiceKind::String => quote!(parsed.as_str()),
        _ => quote!(&parsed)
    }
}

/// Generate match arm for a variant
fn variant_match_arm(variant: &ParsedVariant) -> TokenStream {
    let ident = &variant.ident;
    let span = variant.span;
    let value = match &variant.attribute.value {
        ChoiceValue::String(val) => val.to_token_stream(),
        ChoiceValue::Int(val) => val.to_token_stream(),
        // https://stackoverflow.com/questions/45875142/what-are-the-alternatives-to-pattern-matching-floating-point-numbers
        // https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
        ChoiceValue::Number(val) => quote! { val if (val - #val).abs() < f64::EPSILON }
    };

    quote_spanned! {span=>
         #value => ::std::result::Result::Ok(Self::#ident)
    }
}

/// Generate match arm for a variant in value method
fn value_match_arm(variant: &ParsedVariant) -> TokenStream {
    let ident = &variant.ident;
    let span = variant.span;
    let value = match &variant.attribute.value {
        ChoiceValue::String(val) => val.to_token_stream(),
        ChoiceValue::Int(val) => val.to_token_stream(),
        ChoiceValue::Number(val) => val.to_token_stream()
    };

    quote_spanned! {span=>
        Self::#ident => #value
    }
}