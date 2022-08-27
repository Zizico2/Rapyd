extern crate proc_macro;
use std::{collections::{VecDeque, HashSet}, mem};

use convert_case::{Case, Casing};
use syn::{
    Block,
    parse::Parse,
    parse_macro_input, parse_quote,
    spanned::Spanned,
    token::{Paren, Struct},
    Attribute, Expr, GenericParam, Generics, ItemStruct, LitStr, Local, Path, PathArguments,
    PathSegment, ReturnType, Signature, Stmt, Type, TypePath, Visibility, bracketed, ExprBlock,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use rapyd::*;
use syn::{punctuated::Punctuated, Token};
use syn_rsx::{parse2, Node, NodeName, NodeType};

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
                NodeType::Doctype => panic!("Group"),
                NodeType::Fragment => panic!("Group"),
                NodeType::Block => {
                    let blocks_len = blocks.len();
                    let let_ident = format!("s{}", blocks_len);
                    let let_ident: Ident = Ident::new(&let_ident, Span::mixed_site());
                    let type_ident = {
                        let type_ident = format!("S{}", blocks_len);
                        let type_ident: Ident = Ident::new(&type_ident, Span::mixed_site());
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
        fn_token: Token![fn](Span::mixed_site()),
        ident: parse_quote!(validate),
        generics: parse_quote!(<#generic_params>),
        paren_token: Paren {
            span: Span::mixed_site(),
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

#[proc_macro_attribute]
pub fn always_works_attr(
    _: proc_macro::TokenStream,
    _: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn always_works(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn mock_component(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!().into()
}

#[derive(Debug, Default)]
struct FunctionComponent {
    vis: Option<Visibility>,
    component_name: Option<Ident>,
    hoisted: TokenStream,
    props: Option<ItemStruct>,
    not_hoisted: TokenStream,
    state_idents: HashSet::<String>,
    template: String,
    walks: Vec<Walk>,
    text_code_blocks: Vec<Block>,
    event_code_blocks: Vec<(String, Block)>,
}

impl Parse for FunctionComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func = input.parse::<syn::ItemFn>()?;
        let sig = func.sig;
        let attrs = func.attrs;
        assert!(sig.constness.is_none());
        assert!(sig.abi.is_none());
        assert!(sig.asyncness.is_none()); // Think about this
        assert!(sig.variadic.is_none());
        assert!(sig.unsafety.is_none());
        assert_eq!(sig.output, ReturnType::Default);
        assert!(sig.generics.params.is_empty());
        assert!(sig.generics.where_clause.is_none());
        assert!(sig.generics.lt_token.is_none());
        assert!(sig.generics.gt_token.is_none());
        assert!(attrs.is_empty());

        let mut res = FunctionComponent::default();

        //props
        //TODO differentiate mutable and non mutable props
        let props_punct = sig.inputs;
        res.props = Some(parse_quote!(pub struct Props {#props_punct}));

        // visibility
        res.vis = Some(func.vis);

        // component name
        let ident = sig.ident.to_string();
        res.component_name = Some(Ident::new(&ident, Span::mixed_site()));

        // props
        // ???????????
        // ???????????

        // user-defined code
        let mut contents = func.block.stmts;
        let mut state_factory = StateStructFactory::new();

        //render vars
        let template = &mut res.template;
        let text_code_blocks = &mut res.text_code_blocks;
        let event_code_blocks = &mut res.event_code_blocks;
        let walks = &mut res.walks;
        let render_contents = contents.pop().expect("Component function must not be empty!");

        let state_idents = &mut res.state_idents;
        //
        for stmt in contents {
            match stmt {
                Stmt::Local(ref local) => {
                    if let Some(ref init) = local.init {
                        if let Expr::Macro(ref mac) = *init.1 {
                            let ref attrs = mac.attrs;
                            let ref mac = mac.mac;
                            if mac.path == parse_quote!(state) {
                                assert!(attrs.is_empty());
                                let state = state_factory.next(local.clone());
                                res.hoisted.extend(state.0);
                                let ident = state.1;
                                let ty = state.2;

                                
                                let mut new_local = local.clone();
                                let tokens = &mac.tokens;
                                match &new_local.pat {
                                    syn::Pat::Type(ty) => {
                                        match ty.pat.as_ref() {
                                            syn::Pat::Ident(ident) => {
                                                state_idents.insert(ident.ident.to_string());
                                            },
                                            _ => panic!("_")
                                        }
                                        
                                    },
                                    _ => panic!("_"),
                                }
                                
                                new_local.init = Some((
                                    Token![=](Span::mixed_site()),
                                    Box::new(parse_quote!(
                                        ::rapyd::state::closure::State::<#ty> {
                                            val: ::std::rc::Rc::new(::std::cell::RefCell::new(#ident(#tokens).into())),
                                            reactions: ::std::vec::Vec::new()
                                        }
                                    )),
                                ));
                                match &new_local.pat {
                                    syn::Pat::Type(ref ty) => new_local.pat = *ty.pat.clone(),
                                    _ => panic!("States need explicit types!"),
                                };
                                res.not_hoisted.extend(new_local.into_token_stream());
                                continue;
                            }
                        }
                    }
                }
                Stmt::Item(_) => {}
                Stmt::Expr(_) => {}
                Stmt::Semi(_, _) => {}
            }
            res.not_hoisted.extend(stmt.into_token_stream());
        }

        if let Stmt::Expr(ref expr) = render_contents
        {
            if let Expr::Macro(ref mac) = expr {
                assert!(mac.attrs.is_empty());
                let ref mac = mac.mac;
                if mac.path == parse_quote!(render) {

                    struct NodesWithCloser {
                        nodes: VecDeque<Node>,
                        closer: Option<String>,
                    }

                    let mut nodes_stack = vec![NodesWithCloser {
                        nodes: parse2(mac.tokens.clone()).unwrap().into(),
                        closer: None,
                    }];
                    while let Some(nodes_with_closer) = nodes_stack.last_mut() {
                        let ref mut nodes = nodes_with_closer.nodes;
                        if nodes.is_empty() {
                            if let Some(closer) = &nodes_with_closer.closer {
                                walks.push(Walk::Out(1));
                                template.push_str(&format!("</{}>", closer));
                            }
                            nodes_stack.pop();
                            continue;
                        }
                        while let Some(node) = nodes.pop_front() {
                            for attr in &node.attributes {
                                if let Some(ref name) = attr.name {
                                    if let NodeName::Colon(name) = name {
                                        if name.first().unwrap() == "on" {
                                            event_code_blocks.push((name.last().unwrap().to_string(), attr.value_as_block().unwrap().block))
                                        }
                                    }
                                }
                            }
                            if node.node_type == NodeType::Text {
                                if let Some(value) = node.value_as_string() {
                                    walks.push(Walk::Over(1));
                                    template.push_str(&value);
                                }
                            }
                            else if node.node_type == NodeType::Block {
                                if let Some(value) = node.value_as_block(){
                                    assert!(value.attrs.is_empty());
                                    assert!(value.label.is_none());
                                    text_code_blocks.push(value.block);
                                    template.push_str("<!>");
                                    walks.push(Walk::Replace);
                                    walks.push(Walk::Over(1));
                                }
                            }
                            else if let Some(NodeName::Path(ref name)) = node.name {
                                if let Some(name) = name.path.get_ident() {
                                    let name = name.to_string();
                                    if name.is_case(Case::Pascal) {
                                        template.push_str("<SOME_FOREIGN_COMPONENT>");
                                        //handle children (slots)
                                        continue;
                                    } else {
                                        template.push_str(&format!("<{}>", name));
                                        let children = node.children;
                                        if children.is_empty() {
                                            walks.push(Walk::Over(1));
                                            template.push_str(&format!("</{}>", name));
                                        } else {
                                            walks.push(Walk::Next(1));
                                            nodes_stack.push(NodesWithCloser {
                                                nodes: children.into(),
                                                closer: Some(name),
                                            });
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    panic!("Last Statement must be \"render!\" macro");
                }
            };
        } else {
            panic!("Last Statement must be \"render!\" macro");
        }

        Ok(res)
    }
}

struct StateStructFactory {
    index: u128,
}

impl Default for StateStructFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl StateStructFactory {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self, local: Local) -> (TokenStream, Ident, Type) {
        let ident = format!("State{}", self.index);
        let ident = Ident::new(&ident, Span::mixed_site());
        self.index += 1;

        let ty = match &local.pat {
            syn::Pat::Type(ty) => ty.ty.clone(),
            _ => panic!("States need explicit types!"),
        };
        (
            quote!(
                        pub struct #ident(#ty);
                        impl From<#ident> for #ty {
                            fn from(state: #ident) -> Self {
                                state.0
                            }
                        }
                        impl ::std::ops::Deref for #ident {
                            type Target = #ty;

                            fn deref(&self) -> &Self::Target {
                                &self.0
                            }
                        }
                    
            ),
            ident,
            *ty
        )
    }
}

#[proc_macro_attribute]
pub fn function_component(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let res = parse_macro_input!(item as FunctionComponent);
    let vis = res.vis.unwrap();
    let hoisted = res.hoisted;
    let not_hoisted = res.not_hoisted;
    let props = res.props.unwrap();
    let component_name = res.component_name.unwrap();
    let template = res.template;
    let walks = res.walks;
    let n_walks = walks.len();
    let scoped_n_text_nodes = walks.iter().fold(0, |acc: usize, val| if let Walk::Replace = val {acc + 1} else {acc} );
    let scoped_n_event_targets = walks.iter().fold(0, |acc: usize, val| if let Walk::EventTarget = val  {acc + 1} else {acc});
    let walks = into_token_stream(walks);
    let events = res.event_code_blocks;
    let text_code_blocks = res.text_code_blocks;
    let state_idents = res.state_idents;

    let mut eventttt = TokenStream::default();

    for (i, event) in events.into_iter().enumerate() {
        let event_name = event.0;
        let closure = event.1;
        //Closure<dyn FnMut()>::new("user_provided_code".into());

        eventttt.extend(
            quote!(
                    #[allow(unused_braces)]
                    ::std::ops::Deref::deref(&___scope.0).dom.event_targets[#i]
                    .add_event_listener_with_callback(#event_name, ::wasm_bindgen::closure::Closure::<dyn FnMut()>::new(#closure).as_ref().unchecked_ref())
                    .unwrap();
                )
        );
    }

    let mut state_reactions = TokenStream::default();
    for (i, text) in text_code_blocks.into_iter().enumerate() {
        let func = quote!(move |_| {text_nodes[#i].set_data(&::std::string::ToString::to_string(&#text))});
        let mut states = HashSet::<Ident>::new();
        for stmt in text.stmts {
            match stmt {
                Stmt::Expr(expr) => {
                    let tokens = expr.to_token_stream();
                    for token in tokens {
                        match token {
                            proc_macro2::TokenTree::Ident(ident) => {
                                if state_idents.contains(&ident.to_string()) && !states.contains(&ident) {
                                    state_reactions.extend(quote!(
                                        #ident.push_reaction(::std::rc::Rc::new(::std::cell::RefCell::new(#func.clone())));
                                    ));
                                }
                                states.insert(ident.clone());
                            },
                            _ => {},
                        }
                    }
                },
                _ => {},
            }
        }
    }
    quote!(
        #vis mod #component_name {
            use super::*;
            use ::wasm_bindgen::JsCast;
            #[derive(Debug)]
            #props

            pub const TEMPLATE: &str = #template;
            pub type Walks = [::rapyd::Walk; #n_walks];
            pub const WALKS: Walks = #walks;

            pub const SCOPED_N_TEXT_NODES: usize = #scoped_n_text_nodes;
            //pub const SCOPED_N_TEXT_NODES: usize = #scoped_n_text_nodes;

            pub const SCOPED_N_EVENT_TARGETS: usize = #scoped_n_event_targets;
            //pub const SCOPED_N_EVENT_TARGETS: usize = #scoped_n_event_targets;

            pub const N_TEXT_NODES: usize = SCOPED_N_TEXT_NODES;
            //pub const N_TEXT_NODES: usize = SCOPED_N_TEXT_NODES + #n_text_nodes;

            pub const N_EVENT_TARGETS: usize = SCOPED_N_EVENT_TARGETS;
            //pub const N_EVENT_TARGETS: usize = SCOPED_N_EVENT_TARGETS + #n_event_targets;

            pub type TextNodes = [::web_sys::Text; SCOPED_N_TEXT_NODES];
            pub type EventTargets = [::web_sys::EventTarget; SCOPED_N_EVENT_TARGETS];

            #[derive(Debug)]
            pub struct ChildScopes;


            #[derive(Debug, Clone)]
            pub struct Scope(::std::rc::Rc<::rapyd::Scope<SCOPED_N_TEXT_NODES, SCOPED_N_EVENT_TARGETS, Props, ChildScopes>>);
            
            #hoisted

            impl Scope {
                fn new(props: Props, text_nodes: TextNodes, event_targets: EventTargets) {
                    let mut ___scope =  
                        Scope(::rapyd::Scope::new(
                                props,
                                text_nodes,
                                event_targets,
                                ChildScopes
                            ));
                    #not_hoisted
                    #eventttt
                    #state_reactions
                }
            }
        }
    )
    .into()
}


fn into_token_stream(walks: Vec<Walk>) -> TokenStream {
    let mut tks = TokenStream::default();
    for walk in walks {
        match walk {
            Walk::Next(i) => tks.extend(quote!(::rapyd::Walk::Next(#i),)),
            Walk::Over(i) => tks.extend(quote!(::rapyd::Walk::Over(#i),)),
            Walk::Out(i) => tks.extend(quote!(::rapyd::Walk::Out(#i),)),
            Walk::Replace => tks.extend(quote!(::rapyd::Walk::Replace,)),
            Walk::EventTarget => tks.extend(quote!(::rapyd::Walk::EventTarget,)),
        }
    }

    quote!([#tks])
}


// let handle_click = Closure<dyn FnMut()>::new("user_provided_code".into());
// scope.event_targets[0]
// .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
// .unwrap();