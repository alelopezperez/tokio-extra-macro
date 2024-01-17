use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, StmtMacro};

#[proc_macro_attribute]
pub fn task_spawn(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let old_tokens = tokens.clone();
    let mut item = parse_macro_input!(tokens as ItemFn);

    item.block
        .stmts
        .insert(0, syn::parse(quote!(println!("Start");).into()).unwrap());

    item.to_token_stream().into()
}
