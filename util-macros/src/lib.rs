use proc_macro_error::{abort_if_dirty, emit_error, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Block, ExprClosure,
    LitInt, Stmt,
};

#[proc_macro_error]
#[proc_macro]
pub fn arr(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut closure = parse_macro_input!(item as ExprClosure);

    let mut ts: Punctuated<Block, Comma> = Punctuated::new();

    let size = closure.inputs.pop().unwrap().into_value();
    let size: LitInt = parse_quote!(#size);

    let input_var = closure.inputs.pop().unwrap().into_value();

    for i in 0_usize..size.base10_parse().unwrap() {
        let mut block: Block = parse_quote!({});
        block
            .stmts
            .push(parse_quote!(const #input_var: usize = #i;));

        let iife = Stmt::Expr(parse_quote!((#closure)()));
        block.stmts.push(iife);

        ts.push(block);
    }

    quote!([#ts]).into()
}
