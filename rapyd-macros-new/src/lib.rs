use proc_macro2::TokenStream;
use quote::quote;
use util::{DepthFirstIter, DepthFirstIterNode};

mod util;

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html2(input.into()).into()
}

fn html2(input: TokenStream) -> TokenStream {
    let nodes = syn_rsx::parse2(input).unwrap();
    for DepthFirstIterNode { node, level_diff } in DepthFirstIter::new(nodes.into()) {
        
    }

    todo!()
}
