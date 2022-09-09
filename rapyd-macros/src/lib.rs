extern crate proc_macro;
use std::{collections::{VecDeque, HashSet, HashMap}};

use convert_case::{Case, Casing};
use syn::{
    Block,
    parse::Parse,
    parse_macro_input, parse_quote,
    spanned::Spanned,
    token::{Paren},
    Expr, GenericParam,  ItemStruct, LitStr, Local, Path, PathArguments,
    PathSegment, ReturnType, Signature, Stmt, Type, TypePath, Visibility,   ItemImpl, ImplItemMethod,
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
    index: usize,
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

#[proc_macro_attribute]
pub fn component_struct(_: proc_macro::TokenStream,_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!().into()
}

#[proc_macro_attribute]
pub fn render(_: proc_macro::TokenStream,_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn template(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!(()).into()
}

#[proc_macro_attribute]
pub fn struct_component(_attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: StructComponent = parse_macro_input!(input);
    let state_tags = parsed.state_tags;
    let serializable_scope = parsed.serializable_scope;
    quote!(
        #serializable_scope
        #(#state_tags)*
    ).into()
}

#[derive(Debug, Default)]
struct StateHandleFactory {
    index: usize,
}

impl StateHandleFactory {
    fn next(&mut self, ty: Type, ident: Option<Ident>) -> StateHandle{
        let res = StateHandle {
            ident,
            ty,
            index: self.index,
        };
        self.index += 1;
        res
    }
}


#[derive(Debug)]
struct StateHandle {
    ident: Option<Ident>,
    ty: Type,
    index: usize,
}

impl StateHandle {
    fn get_type(&self) -> Type {
        let ty = &self.ty;
        let ident = Ident::new(&format!("___StateTag{}", self.index), Span::mixed_site());
        let res = parse_quote!(::rapyd::state::State<#ty, #ident, ___Scope>);
        res
    }
}

impl ToTokens for StateHandle  {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ref ty = self.ty;
        let tag_ident: Ident = Ident::new(&format!("___StateTag{}", self.index), Span::mixed_site());
        let index = self.index;
        let change_method_ident = self.ident.as_ref().map(|f| f.to_string()).unwrap_or(index.to_string());
        let change_method_ident = Ident::new(&format!("___on_change_{}", change_method_ident), Span::mixed_site());

        
        quote!(
            struct #tag_ident(#ty);
            impl ::rapyd::state::StateTag<#ty> for #tag_ident {}
            impl ::std::convert::From<#tag_ident> for #ty {
                fn from(val: #tag_ident) -> Self {
                    val.0
                }
            }
            impl ::rapyd::state::UpdateState<#tag_ident, #ty> for ___Scope {
                fn update_state(&mut self, new: &#ty) {
                    self.#change_method_ident();
                }
            }

        ).to_tokens(tokens);
    }
}

#[derive(Debug)]
struct StructComponent {
    serializable_scope: ItemStruct,
    state_tags: Vec<StateHandle>,
}

impl Parse for StructComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut state_tags: Vec<StateHandle> = Default::default();

        let mut item_struct = input.parse::<ItemStruct>()?;

        let mut state_factory = StateHandleFactory::default();
        for field in item_struct.fields.iter_mut() {
            if let Visibility::Inherited = field.vis {} else {
                panic!("Component instance variables should have no visibility modifier!");
            }
            if field.attrs.first() == Some(&parse_quote!(#[state])) {
                let _attr = field.attrs.pop().unwrap();

                if !field.attrs.is_empty() {
                    panic!("State variables shouldn't have any extra attributes!");
                }

                let handle = state_factory.next(field.ty.clone(), field.ident.clone());
                field.ty = handle.get_type();
                state_tags.push(handle);
            }
        }
        item_struct.ident = parse_quote!(___SerializableScope);
        Ok(Self {
            state_tags,
            serializable_scope: item_struct,
        })
    }
}


// ---------------------------------------------------------------
// ---------------------------------------------------------------
// ---------------------------------------------------------------
// ---------------------------------------------------------------



#[proc_macro_attribute]
pub fn struct_component_impl(_attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: StructComponentImpl = parse_macro_input!(input);
    let n_text_nodes = parsed.n_text_nodes;
    let text_node_methods = parsed.text_node_methods;
    let on_change_methods = parsed.on_change_methods.values();
    let template = parsed.template;
    let walks_len = parsed.walks.len();
    let walks = into_token_stream(parsed.walks);
    //let state_tags = parsed.state_tags;
    //let serializable_scope = parsed.serializable_scope;
    quote!(
        #[derive(Clone)]
        struct ___Scope {
            text_nodes: [::web_sys::Text; #n_text_nodes],
            serializable_scope: ::std::rc::Rc<___SerializableScope>,
        }
        impl ___Scope {
            const TEMPLATE: &'static str = #template;
            const WALKS: [::rapyd::Walk; #walks_len] = #walks;
            #(#on_change_methods)*
        }
        impl ___SerializableScope {
            #(#text_node_methods)*
        }
    ).into()
}

struct StructComponentImpl {
    n_text_nodes: usize,
    text_node_methods: Vec<ImplItemMethod>,
    //registered_state_vars: Vec<Ident>,
    on_change_methods: HashMap<Ident,  ImplItemMethod>,
    template: String,
    walks: Vec<Walk>,
}

impl Parse for StructComponentImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let borrow_regex = fancy_regex::Regex::new(r"borrow!\s\(.+?(?=\))").unwrap();
        let mut n_text_nodes = 0;
        let mut text_node_methods: Vec<ImplItemMethod> = Default::default();
        //let mut registered_state_vars: HashSet<Ident> = Default::default();
        let mut on_change_methods: HashMap<Ident, ImplItemMethod> = Default::default();
        let mut output_array = Punctuated::<Expr, Token![,]>::default();
        let mut template_stmt = 0;
        let mut template = String::new();
        let mut walks = Vec::<Walk>::new();
        let mut closing_tags = Vec::<String>::new();
        let mut blocks = String::new();

        let item_impl = input.parse::<ItemImpl>()?;
        for item in item_impl.clone().items {
            match item {
                syn::ImplItem::Method(method) => {
                    let render_ident: Ident = parse_quote!(render);
                    if method.sig.ident == render_ident {
                        let mut template_macro_found = false;
                        for (i, stmt) in method.block.stmts.iter().enumerate() {
                            match stmt {
                                Stmt::Expr(expr) => {
                                    match expr {
                                        Expr::Macro(mac) => {
                                            if mac.mac.path == parse_quote!(template) {
                                                if template_macro_found {
                                                    panic!("The template macro should only be called once!");
                                                } else {
                                                    template_macro_found = true;
                                                }
                                                let nodes = parse2(mac.mac.tokens.clone()).unwrap();
                                                for DepthFirstIterNode {node, level_diff} in into_depth_first_iterator(nodes)  {
                                                    for _ in 0..-level_diff {
                                                        let tag = closing_tags.pop().expect("No more closing tags!");
                                                        walks.push(Walk::Out(1));
                                                        template.push_str(&tag);    

                                                    }
                                                    if node.node_type == NodeType::Element {
                                                        let node_name = node.name.unwrap().to_string();
                                                        template.push_str(&format!("<{}>", node_name));
                                                        walks.push(Walk::Next(1));
                                                        closing_tags.push(format!("</{}>", node_name));
                                                    }
                                                    else if let Some(block) = node.value_as_block() {
                                                        template.push_str("<!>");
                                                        walks.push(Walk::Replace);
                                                        walks.push(Walk::Over(1));
                                                        let mut method = method.clone();
                                                        method.attrs.push(parse_quote!(#[allow(unused_braces)]));
                                                        method.attrs.push(parse_quote!(#[allow(unused_variables)]));
                                                        method.sig.output = parse_quote!( -> ::std::string::String);
                                                        method.sig.ident = Ident::new(&format!("text_node_rerender_{}", n_text_nodes), Span::mixed_site());
                                                        let out: Expr = parse_quote!(::rapyd::state::ToTextData::to_text_data(#block));
                                                        output_array.push(out.clone());
                                                        method.block.stmts[i] = Stmt::Expr(out);
                                                        template_stmt = i;
                                                        text_node_methods.push(method.clone());
                                                        
                                                                                
                                                        let block_str = block.block.to_token_stream().to_string();

                                                        //TODO REGEX SHENENIGANS - THIS NEEDS TO BE REWORKED FOR SURE
                                                        blocks.push_str(&format!("{}\n", block_str));
                                                        let matches = borrow_regex.find_iter(&block_str).map(|m| {
                                                                let m = m.unwrap().as_str();
                                                                let ident: &str = &m[9..m.len()];
                                                                Ident::new(ident, Span::mixed_site())
                                                        });
                                                        
                                                        for state_ident in matches {
                                                                                let ident = Ident::new(&format!("___on_change_{}", state_ident.to_string()), Span::mixed_site());
                                                                                let inner_ident = method.clone().sig.ident;
                                                                                match on_change_methods.get_mut(&ident) {
                                                                                    Some(method) => {
                                                                                        method.block.stmts.push(
                                                                                            parse_quote!(
                                                                                                {
                                                                                                    let ref text = self.text_nodes[#n_text_nodes];
                                                                                                    let new_data = self.serializable_scope.#inner_ident();
                                                                                                    if text.data() != new_data {
                                                                                                        text.set_data(&new_data);
                                                                                                    }
                                                                                                }
                                                                                            )
                                                                                        );
                                                                                    },
                                                                                    None => {
                                                                                        let method = ImplItemMethod {
                                                                                            attrs: Default::default(),
                                                                                            vis: parse_quote!(pub),
                                                                                            defaultness: None,
                                                                                            sig: parse_quote!(fn #ident(&mut self)),
                                                                                            block: parse_quote!(
                                                                                                {
                                                                                                    {
                                                                                                        let ref text = self.text_nodes[#n_text_nodes];
                                                                                                        let new_data = self.serializable_scope.#inner_ident();
                                                                                                        if text.data() != new_data {
                                                                                                            text.set_data(&new_data);
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            )
                                                                                        };
                                                                                        // let input_str = input.to_string();
                                                                                        on_change_methods.insert(ident, method);
                                                                                    }
                                                                                }
                                                                            }

                                                        /* HANDLE ___on_changes */
                                                        /*
                                                        ___on_change_count
                                                        */
                                                        n_text_nodes += 1;
                                                    }
                                                }
                                            }
                                        },
                                        _ => {},
                                    }
                                },
                                _ => {},
                            }
                        }
                        let mut method = method.clone();
                        method.attrs.push(parse_quote!(#[allow(unused_braces)]));
                        method.attrs.push(parse_quote!(#[allow(unused_variables)]));
                        method.sig.output = parse_quote!( -> [::std::string::String; #n_text_nodes]);
                        method.sig.ident = Ident::new("text_node_rerender_all", Span::mixed_site());
                        let out: Expr = parse_quote!([#output_array]);
                        output_array.push(out.clone());
                        method.block.stmts[template_stmt] = Stmt::Expr(out);
                        text_node_methods.push(method.clone());
                    }
                },
                _ => {},
            }
        }
        
        walks.push(Walk::Out(closing_tags.len()));
        for tag in closing_tags {
            template.push_str(&tag);
        }
         
        Ok(StructComponentImpl {
            n_text_nodes,
            text_node_methods,
            on_change_methods,
            template,
            walks,
        })
    }
}


#[proc_macro]
pub fn borrow(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    borrow2(input.into()).into()
}


fn borrow2(input: TokenStream) -> TokenStream {
    //quote!(::std::ops::Deref::deref(self.#input.borrow()))
    quote!(&*self.#input.borrow())
}

#[proc_macro]
pub fn dead_code(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dead_code2(input.into()).into()
}

fn dead_code2(input: TokenStream) -> TokenStream {
    quote!(
        if false {
            #input.borrow()
        }
    )
}

fn into_depth_first_iterator(val: Vec<Node>) -> DepthFirstIter{
    DepthFirstIter {
        nodes_stack: vec![val.into()]
    }
}

struct DepthFirstIter {
    nodes_stack: Vec<VecDeque<Node>>,
}

struct DepthFirstIterNode {
    pub node: Node,
    pub level_diff: isize,
}

impl Iterator for DepthFirstIter {
    type Item = DepthFirstIterNode;

    fn next(&mut self) -> Option<Self::Item> {
        let old_stack_len = self.nodes_stack.len();
        let mut nodes = None;
        let mut level_diff = None;
        while let Some(inner_nodes) = self.nodes_stack.last() {
            if !inner_nodes.is_empty() {
                level_diff = Some((self.nodes_stack.len() as isize) - (old_stack_len as isize));
                nodes = self.nodes_stack.last_mut();
                break;
            }
            self.nodes_stack.pop();
        }

        if let Some(nodes) = nodes {
            if let Some(node) = nodes.pop_front() {
                if node.node_type == NodeType::Element {
                    self.nodes_stack.push(node.children.into());
                }
                return Some(
                    DepthFirstIterNode {
                        node: Node {
                            children: vec![],
                            ..node
                        },
                        level_diff: level_diff.expect("level_diff not set!")
                    }
                )
            }
        }
        None
    }
    
}


//---------------
//--------------- Module based
//---------------

