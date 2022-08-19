extern crate proc_macro;
use std::collections::VecDeque;

use syn::{
    parse_quote, spanned::Spanned, token::Paren, GenericParam, LitStr, Path, PathArguments,
    PathSegment, Signature, Stmt, Type, TypePath,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{punctuated::Punctuated, Token};
use syn_rsx::{parse2, Node, NodeType};

#[proc_macro]
pub fn component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component2(TokenStream::from(input)).into()
}

fn component2(input: TokenStream) -> TokenStream {
    let nodes = parse2(input).unwrap();
    let mut script_block = Default::default();
    let mut template = None;
    let mut sig = None;
    let mut blocks = None;
    let mut state_idents = None;

    for node in nodes {
        match node.name_as_string() {
            Some(node_name) => {
                match node_name.as_str() {
                    "script" => {
                        script_block = node.children[0].value_as_block();
                    }
                    "template" => {
                        let (inner_template, inner_sig, inner_blocks, inner_state_idents) =
                            process_template(node.children.into());
                        template = Some(inner_template);
                        sig = Some(inner_sig);
                        blocks = Some(inner_blocks);
                        state_idents = Some(inner_state_idents);
                    }
                    "style" => {}
                    _ => {}
                };
            }
            None => {}
        }
    }

    let template = template.expect("WHERE'S MY TEMPLATE");
    let sig = sig.expect("WHERE'S MY SIGNATURE");
    let blocks = blocks.expect("WHERE'RE MY BLOCKS");
    let state_idents = state_idents.expect("WHERE'RE MY STATE IDENTIFICATIONS");

    let script_expr = match script_block {
        Some(script_block) => {
            let mut stmts = TokenStream::new();
            for stmt in script_block.block.stmts {
                stmt.to_tokens(&mut stmts);
            }
            quote!(
                pub mod component {
                    #sig {
                        (#state_idents)
                    }
                    pub struct Component;
                    impl Component
                    {
                        pub fn new() -> Self {
                            Self
                        }
                        pub fn render(&self) -> String {
                            #stmts
                            #[allow(unused_braces)]
                            let (#state_idents) = validate(#blocks);
                            format!(concat!(#template), #state_idents)
                        }
                    }
                }
            )
        }
        None => panic!("MISSING SCRIPT BLOCK"),
    };

    script_expr
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn process_template(
    nodes: VecDeque<Node>,
) -> (
    Punctuated<LitStr, Token![,]>,
    Signature,
    Punctuated<OptionalStatement, Token![,]>,
    Punctuated<Ident, Token![,]>,
) {
    let mut blocks: Punctuated<OptionalStatement, Token![,]> = Punctuated::new();

    let mut generic_params: Punctuated<GenericParam, Token![,]> = Punctuated::new();
    let mut inputs = Punctuated::new();
    let mut return_tuple_elements: Punctuated<Type, Token![,]> = Punctuated::new();
    let mut template = Punctuated::new();
    let mut state_idents = Punctuated::new();

    let mut stack: Vec<VecDeque<Node>> = vec![nodes].into();
    let mut closing_tags = vec![];

    let mut going_deeper = false;

    while let Some(mut nodes) = stack.pop() {
        while let Some(node) = nodes.pop_front() {
            going_deeper = false;
            match node.node_type {
                NodeType::Element => {
                    let tag_name = node.name_as_string().unwrap();
                    let opening_tag = format!("<{}>", &tag_name);
                    template.push(parse_quote!(#opening_tag));
                    let closing_tag = format!("</{}>", &tag_name);
                    closing_tags.push(parse_quote!(#closing_tag));
                    if !node.children.is_empty() {
                        stack.push(nodes);
                        stack.push(node.children.into());
                        going_deeper = true;
                        break;
                    } else {
                        template.push(closing_tags.pop().unwrap());
                    }
                }
                NodeType::Attribute => panic!("Attributes should be handled separately!"),
                NodeType::Text => {
                    let text = node.value_as_string().unwrap();
                    template.push(parse_quote!(#text));
                }
                NodeType::Comment => {
                    let text = format!("<!-- {} -->", node.value_as_string().unwrap());
                    template.push(parse_quote!(#text));
                }
                NodeType::Doctype => todo!(),
                NodeType::Fragment => todo!(),
                NodeType::Block => {
                    let blocks_len = blocks.len();
                    let let_ident = format!("s{}", blocks_len);
                    let let_ident: Ident = Ident::new(&let_ident, Span::call_site());
                    let type_ident = {
                        let type_ident = format!("S{}", blocks_len);
                        let type_ident: Ident = Ident::new(&type_ident, Span::call_site());
                        let mut segments: Punctuated<PathSegment, Token![::]> = Punctuated::new();
                        segments.push(PathSegment {
                            ident: type_ident,
                            arguments: PathArguments::None,
                        });
                        let path: Path = Path {
                            leading_colon: None,
                            segments,
                        };
                        Type::Path(TypePath { qself: None, path })
                    };
                    let block = node.value_as_block().unwrap();
                    //blocks
                    if block.block.stmts.len() > 1 {
                        blocks.push(OptionalStatement::None(block.block.span()));
                    } else {
                        blocks.push(OptionalStatement::Some(block.block.stmts[0].clone()));
                    }
                    //generic_params
                    generic_params.push(parse_quote!(#type_ident: std::fmt::Display));
                    //inputs
                    inputs.push(parse_quote!(#let_ident: #type_ident));
                    //return_tuple_elements
                    return_tuple_elements.push(type_ident);
                    //state_idents
                    state_idents.push(let_ident);
                    //template
                    template.push(parse_quote!("{}"));
                }
            }
        }
        if !going_deeper {
            if let Some(tag) = closing_tags.pop() {
                template.push(tag);
            }
        }
    }

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](Span::call_site()),
        ident: parse_quote!(validate),
        generics: parse_quote!(<#generic_params>),
        paren_token: Paren {
            span: Span::call_site(),
        },
        inputs,
        variadic: None,
        output: parse_quote!(-> (#return_tuple_elements)),
    };

    (template, sig, blocks, state_idents)
}

// what can affect rerendering
// changes in state_vars (triggered by event listeners)
// changes in props (triggered by the parent component)
// changes in a subscribed store/context (triggered by anyone)

enum OptionalStatement {
    Some(Stmt),
    None(Span),
}

impl ToTokens for OptionalStatement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OptionalStatement::Some(ref stmt) => stmt.to_tokens(tokens),
            OptionalStatement::None(span) => {
                let span = span.clone();
                quote_spanned!(
                span =>
                {compile_error!("Blocks should only have 1 statement!")})
                .to_tokens(tokens)
            }
        }
    }
}

#[proc_macro_attribute]
pub fn do_nothing(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
