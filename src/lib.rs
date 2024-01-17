use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote_spanned, spanned::Spanned, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn background_task(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(tokens as ItemFn);

    // Change `async fn()` to `fn()`
    item.sig.asyncness = None;
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = item;

    // Change fn
    match &sig.output {
        ReturnType::Default => {}
        ReturnType::Type(_, ret) => {
            let ret = quote!(#ret);
            sig.output = parse_quote_spanned! {ret.span()=>
                -> tokio::task::JoinHandle<#ret>
            };
        }
    };

    let stmts = &block.stmts;

    let new_tokens = match &sig.output {
        ReturnType::Default => {
            quote! {
                #(#attrs)* #vis #sig {
                    tokio::spawn(async {
                        #(#stmts)*
                    });
                }
            }
        }
        ReturnType::Type(_, _) => {
            quote! {
                #(#attrs)* #vis #sig {
                    tokio::spawn(async {
                        #(#stmts)*
                    })
                }
            }
        }
    };

    new_tokens.into()
}
