use crate::util;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse2, spanned::Spanned, Error, ItemFn, Result};

pub fn error_handler(input: TokenStream2) -> Result<TokenStream2> {
    let fun = parse2::<ItemFn>(input)?;
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block
    } = fun;

    match sig.inputs.len() {
        c if c != 2 => {
            // This hook is expected to have three arguments, a reference to an `SlashContext`,
            // a &str indicating the name of the command and the result of a command execution.
            return Err(Error::new(sig.inputs.span(), "Expected two arguments"));
        }
        _ => ()
    };

    // The name of the original function
    let ident = sig.ident;
    // This is the name the given function will have after this macro's execution
    let fn_ident = quote::format_ident!("_{}", &ident);
    sig.ident = fn_ident.clone();

    /*
    Check the return of the function, returning if it does not match, this function is required
    to return `()`
    */
    util::check_return_type(&sig.output, quote::quote!(()))?;

    let error_type = util::get_path(&util::get_pat(sig.inputs.iter().nth(1).unwrap())?.ty, false)?;

    let ty = util::get_context_type(&sig, true)?;
    // Get the hook macro so we can fit the function into a normal fn pointer
    let hook = util::get_hook_macro();
    let path = quote::quote!(::zephyrus::hook::ErrorHandlerHook);

    Ok(quote::quote! {
        pub fn #ident() -> #path<#ty, #error_type> {
            #path(#fn_ident)
        }

        #[#hook]
        #(#attrs)*
        #vis #sig #block
    })
}
