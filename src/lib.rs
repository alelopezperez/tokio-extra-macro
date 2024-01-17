use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn background_task(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(tokens as ItemFn);

    item.sig.asyncness = None;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;

    let stmts = &block.stmts;

    let new = quote! {
        #(#attrs)* #vis #sig {
            tokio::spawn(async {
                #(#stmts)*
            });
        }
    };

    new.into()
}
