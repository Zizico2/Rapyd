use proc_macro2::{Span, TokenStream};
use proc_macro_error::{emit_error, proc_macro_error};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem,
};
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Comma,
    visit::Visit,
    visit_mut::VisitMut,
    Attribute, Error, Expr, ExprCall, ExprClosure, ExprLet, ExprMacro, ExprPath, Field, Ident,
    ImplItem, ImplItemMethod, Item, ItemFn, ItemImpl, ItemMacro, ItemMod, ItemStruct, Local, Macro,
    MacroDelimiter, Member, Meta, Pat, Path, Token,
};
use syn_rsx::parse2;
mod scope_field_attrs;
mod scope_struct;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn state(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn prop(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

struct ComponentVisitor {
    pub scope_visitor: ScopeVisitor,
}

impl VisitMut for ComponentVisitor {
    fn visit_item_struct_mut(&mut self, item_struct: &mut ItemStruct) {
        if item_struct.ident == format_ident!("Scope") {
            match &item_struct.fields {
                syn::Fields::Named(_) => {
                    self.scope_visitor.visit_item_struct_mut(item_struct);
                }
                _ => emit_error!(item_struct, "Should have named fields!"),
            }
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn test_use_attr(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut component = parse_macro_input!(item as ItemMod);
    /*
    let scope = component.content.as_mut().unwrap().1.pop().unwrap();
    let mut scope = match scope {
        Item::Struct(scope) => scope,
        _ => todo!(),
    };
    */

    let mut component_visitor = ComponentVisitor {
        scope_visitor: ScopeVisitor {
            scope_impl: None,
            state_fields: HashMap::new(),
            prop_fields: HashMap::new(),
        },
    };
    component_visitor.visit_item_mod_mut(&mut component);

    /*
    match &mut scope.fields {
        syn::Fields::Named(fields) => visitor.visit_fields_named_mut(fields),
        _ => todo!(),
    }
    let scope_ident = scope.ident.clone();

    component.content.as_mut().unwrap().1.push(scope.into());
    */

    component_visitor
        .scope_visitor
        .scope_impl
        .and_then(|item_impl| {
            component.content.as_mut().unwrap().1.push(item_impl.into());
            Some(())
        });

    quote! {
        //#component
    }
    .into()
}
#[derive(Debug)]
struct StateOptions {
    pub state_tag_ident: Ident,
    pub source: Option<Attribute>,
}
impl StateOptions {
    fn new_without_source(state_tag_ident: Ident) -> Self {
        StateOptions {
            state_tag_ident,
            source: None,
        }
    }
}

#[derive(Debug, Default)]
struct PropOptions {
    source: Option<Attribute>,
}

//struct

struct ScopeVisitor {
    //impl ::rapyd::Scope for #scope_ident {})
    scope_impl: Option<ItemImpl>,
    state_fields: HashMap<Ident, StateOptions>,
    prop_fields: HashMap<Ident, PropOptions>,
}

impl ScopeVisitor {}

impl VisitMut for ScopeVisitor {
    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        let ident = &i.ident;
        self.scope_impl = Some(parse_quote!(impl ::rapyd::Scope for #ident {}));
        self.visit_fields_mut(&mut i.fields);
    }
    fn visit_field_mut(&mut self, field: &mut Field) {
        let mut attrs = vec![];
        mem::swap(&mut field.attrs, &mut attrs);
        for attr in attrs {
            match (&attr).try_into() {
                Ok(scope_field) => match scope_field {
                    ScopeFieldAttribute::State => {
                        let ident = field.ident.clone().expect("Should be named field!");
                        let state_tag_ident = format_ident!("StateTag{}", ident);

                        if !self.state_fields.contains_key(&ident) {
                            self.state_fields.insert(
                                ident,
                                StateOptions {
                                    source: Some(attr),
                                    state_tag_ident,
                                },
                            );
                        } else {
                            emit_error!(attr, "Duplicate #[state] attribute!");
                            self.state_fields
                                .insert(ident, StateOptions::new_without_source(state_tag_ident));
                        }

                        //field.ty
                    }
                    ScopeFieldAttribute::Prop => {
                        let key = field.ident.clone().expect("Should be named field!");
                        if !self.prop_fields.contains_key(&key) {
                            self.prop_fields
                                .insert(key, PropOptions { source: Some(attr) });
                        } else {
                            self.prop_fields.insert(key, PropOptions::default());
                        }
                    }
                },
                Err(_error) => field.attrs.push(attr),
            }
        }
    }
}

enum ScopeFieldAttribute {
    State,
    Prop,
}

enum AttributeIntoScopeFieldError {
    NotAMetaAttribute,
    NotAScopeFieldAttribute,
    NotAPathAttribute,
}

impl<'a> TryInto<ScopeFieldAttribute> for &'a Attribute {
    type Error = AttributeIntoScopeFieldError;

    fn try_into(self) -> Result<ScopeFieldAttribute, Self::Error> {
        match self.parse_meta() {
            Ok(meta) => match &meta {
                Meta::Path(path) => {
                    if is_state_attr_path(path) {
                        return Ok(ScopeFieldAttribute::State);
                    } else if is_prop_attr_path(path) {
                        return Ok(ScopeFieldAttribute::Prop);
                    } else {
                        // emit_error!(meta, "Valid attributes: #[state], #[prop]");
                        return Err(AttributeIntoScopeFieldError::NotAScopeFieldAttribute);
                    }
                }
                _meta => {
                    // emit_error!(meta, "Not a path attribute");
                    return Err(AttributeIntoScopeFieldError::NotAPathAttribute);
                }
            },
            Err(_error) => {
                // emit_error!(error.span(), "{}", error.to_string());
                return Err(AttributeIntoScopeFieldError::NotAMetaAttribute);
            }
        }
    }
}

fn is_state_attr_path(path: &Path) -> bool {
    *path == parse_quote!(state)
}

fn is_prop_attr_path(path: &Path) -> bool {
    *path == parse_quote!(prop)
}

struct Component {
    ident: Ident,
    content: Vec<Item>,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let content = match module.content {
            Some(content) => content.1,
            None => {
                return Err(Error::new_spanned(
                    module,
                    "Component module must have content!",
                ))
            }
        };
        let component = Component {
            ident: module.ident,
            content,
        };
        Ok(component)
    }
}

fn combine_error_option(error: Option<Error>, new_error: Error) -> Option<Error> {
    match error {
        Some(mut error) => {
            error.combine(new_error);
            Some(error)
        }
        None => Some(new_error),
    }
}

fn is_html_macro_path(path: &Path) -> bool {
    *path == parse_quote!(html)
}

fn is_css_macro_path(path: &Path) -> bool {
    *path == parse_quote!(css)
}

enum RapydTopLevelMacro {
    Html(ItemMacro),
    Css(ItemMacro),
}

impl TryFrom<ItemMacro> for RapydTopLevelMacro {
    // TODO think of some error
    type Error = ();

    fn try_from(value: ItemMacro) -> Result<Self, Self::Error> {
        match value.mac.delimiter {
            MacroDelimiter::Brace(_) => {}
            _ => return Err(()),
        }
        if !value.attrs.is_empty() || value.ident.is_some() || value.semi_token.is_some() {
            return Err(());
        }

        if is_html_macro_path(&value.mac.path) {
            return Ok(RapydTopLevelMacro::Html(value));
        } else if is_css_macro_path(&value.mac.path) {
            return Ok(RapydTopLevelMacro::Css(value));
        }

        Err(())
    }
}
fn take_scope_attribute(item_struct: &mut ItemStruct) -> Vec<Attribute> {
    let mut full_attrs = vec![];
    mem::swap(&mut full_attrs, &mut item_struct.attrs);

    let (scope_attrs, other_attrs): (Vec<_>, Vec<_>) = full_attrs
        .into_iter()
        .partition(|attr| attr.path == parse_quote!(scope));

    item_struct.attrs = other_attrs;

    scope_attrs
}

fn component2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let component: ItemMod = parse_quote!(#item);
    let content = component
        .content
        .expect("This should be a module declaration!")
        .1;

    let mut scope_structs_with_attr: Vec<(Attribute, ItemStruct)> = vec![];
    let mut scope_structs_without_attr: Vec<ItemStruct> = vec![];
    let mut invalid_scope_structs_from_attr: Vec<ItemStruct> = vec![];
    let mut invalid_scope_structs_from_name: Vec<ItemStruct> = vec![];

    let mut everything_else = TokenStream::new();
    let mut errors = vec![];

    for item in content {
        match item {
            Item::Const(_) => todo!(),
            Item::Enum(_) => todo!(),
            Item::ExternCrate(_) => todo!(),
            Item::Fn(_) => todo!(),
            Item::ForeignMod(_) => todo!(),
            Item::Impl(_) => todo!(),
            Item::Macro(_) => todo!(),
            Item::Macro2(_) => todo!(),
            Item::Mod(_) => todo!(),
            Item::Static(_) => todo!(),
            Item::Struct(mut item_struct) => {
                let mut scope_attrs = take_scope_attribute(&mut item_struct);
                if !scope_attrs.is_empty() {
                    match scope_attrs.len() {
                        1 => {
                            let attr = scope_attrs.pop().unwrap();
                            // TODO parse_scope_attr should be a function with better validation than this
                            if &attr == &parse_quote!(#[scope]) {
                                scope_structs_with_attr.push((attr, item_struct));
                            } else {
                                errors.push(Error::new_spanned(attr, "Invalid `scope` attribute!"));
                                invalid_scope_structs_from_attr.push(item_struct);
                            }
                        }
                        _ => {
                            errors.extend(scope_attrs.iter().map(|attr| {
                                Error::new_spanned(attr, "Found multiple `scope` attributes!")
                            }));

                            errors.extend(scope_attrs.iter().filter_map(|attr| {
                                // TODO parse_scope_attr should be a function with better validation than this
                                if attr != &parse_quote!(#[scope]) {
                                    Some(Error::new_spanned(attr, "Invalid `scope` attribute!"))
                                } else {
                                    None
                                }
                            }));

                            invalid_scope_structs_from_attr.push(item_struct);
                        }
                    }
                    continue;
                }
                let scope_ident: Ident = parse_quote!(Scope);

                if item_struct.ident == scope_ident {
                    scope_structs_without_attr.push(item_struct);
                    continue;
                }
                everything_else.extend(item_struct.to_token_stream());
            }
            Item::Trait(_) => todo!(),
            Item::TraitAlias(_) => todo!(),
            Item::Type(_) => todo!(),
            Item::Union(_) => todo!(),
            Item::Use(_) => todo!(),
            Item::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }

    /*
    quote! {
        #errors
        #[allow(non_snake_case)]
        mod #mod_ident {
                #(#update_methods)*
                #scope_struct

                pub mod __html_template {
                    pub const TEMPLATE: &str = #template;
                    #scope_impl
                }
            #everything_else
        }
    }
    */

    quote! {
        /*
        #mod_vis mod #mod_ident {
            //#scope_struct
            #(#errors)*
            #everything_else
        }
        */
    }
}

#[derive(Debug)]
struct FindDependencies {
    scope_vars: HashSet<Ident>,
    state_vars: HashSet<Ident>,
    pub state_dependencies: HashSet<Ident>,
    pub scope_dependencies: HashSet<Ident>,
    pub errors: TokenStream,
}
impl FindDependencies {
    pub fn new(scope_vars: HashSet<Ident>, state_vars: HashSet<Ident>) -> Self {
        Self {
            scope_vars,
            state_vars,
            state_dependencies: Default::default(),
            scope_dependencies: Default::default(),
            errors: Default::default(),
        }
    }
}
impl VisitMut for FindDependencies {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        if self.scope_vars.contains(i) {
            self.scope_dependencies.insert(i.clone());
        }
        if self.state_vars.contains(i) {
            self.state_dependencies.insert(i.clone());
        }
    }
    fn visit_path_mut(&mut self, i: &mut Path) {
        let segments_len = i.segments.len();
        let first_segment_ident = i.segments.first_mut().expect("This should never happen");

        if segments_len == 1 {
            self.visit_path_segment_mut(first_segment_ident);
            return;
        }
        let super_ident: Ident = Token![super](Span::mixed_site()).into();
        let crate_ident: Ident = Token![crate](Span::mixed_site()).into();

        if i.leading_colon.is_none()
            && first_segment_ident.ident != super_ident
            && first_segment_ident.ident != crate_ident
        {
            let error = Error::new_spanned(
                i.to_token_stream(),
                "Local paths must start with \"super\", \"crate\" or \"::\".",
            );

            self.errors.extend(error.to_compile_error());
            return;
        }

        for segment in &mut i.segments {
            self.visit_path_segment_mut(segment);
        }
    }

    fn visit_expr_call_mut(&mut self, i: &mut ExprCall) {
        let func = &mut i.func;
        if let Expr::Path(path) = &mut **func {
            if let Some(ident) = path.path.get_ident() {
                i.func = parse_quote!(Self::#ident);
                let args = &i.args;
                i.args = parse_quote!(&self, #args);
            } else {
                self.visit_expr_path_mut(path);
            }
        }
    }

    fn visit_local_mut(&mut self, i: &mut Local) {
        let error = Error::new_spanned(i.to_token_stream(), "Local let bingings not allowed");
        self.errors.extend(error.to_compile_error());
    }
}

struct HtmlMacroOutput {
    pub template: String,
    pub text_methods: Vec<ImplItemMethod>,
    pub errors: TokenStream,
    pub update_methods: Vec<ItemFn>,
}

struct ScopeStructOutput {
    scope_struct: ItemStruct,
    state_idents: HashSet<Ident>,
    scope_idents: HashSet<Ident>,
}
// TODO: I don't like this name
fn process_scope_struct(mut scope_struct: ItemStruct) -> ScopeStructOutput {
    let mut state_idents: HashSet<Ident> = HashSet::new();
    let mut scope_idents: HashSet<Ident> = HashSet::new();
    for field in scope_struct.fields.iter_mut() {
        let mut i = 0;
        loop {
            let current_i = i;
            i += 1;

            if let Some(attr) = field.attrs.get(current_i) {
                if attr.path == parse_quote!(state) {
                    state_idents.insert(field.ident.as_ref().expect("Gimme names!").clone());
                    field.attrs.remove(current_i);
                    i -= 1;
                } else if attr.path == parse_quote!(prop) {
                    scope_idents.insert(field.ident.as_ref().expect("Gimme names!").clone());
                    field.attrs.remove(current_i);
                    i -= 1;
                }
            } else {
                break;
            }
        }
    }
    ScopeStructOutput {
        scope_struct,
        state_idents,
        scope_idents,
    }
}

// TODO: I don't like this name
fn process_html_macro(
    html_macro: ItemMacro,
    state_vars: HashSet<Ident>,
    scope_vars: HashSet<Ident>,
) -> HtmlMacroOutput {
    let mut template = String::new();
    let mut text_methods: Vec<ImplItemMethod> = vec![];
    let mut update_methods: Vec<ItemFn> = vec![];
    let mut errors = TokenStream::new();
    {
        let mut nodes_stack = vec![VecDeque::from(
            parse2(html_macro.mac.tokens).expect("Bad HTML!"),
        )];

        let mut closing_tags = vec![];

        while let Some(mut nodes) = nodes_stack.pop() {
            let mut going_deeper = false;
            let mut nodes_is_empty = true;
            while let Some(node) = nodes.pop_front() {
                nodes_is_empty = nodes.is_empty();
                match node {
                    syn_rsx::Node::Element(element) => {
                        match element.name {
                            syn_rsx::NodeName::Path(path) => {
                                // TODO handle components here (names starting with capital letters)
                                let tag_name = path
                                    .path
                                    .get_ident()
                                    .expect("Tag name should be ident!")
                                    .to_string();

                                let children = element.children;
                                if children.is_empty() {
                                    template.push_str(&format!("<{0}></{0}>", tag_name));
                                } else {
                                    going_deeper = true;
                                    template.push_str(&format!("<{}>", tag_name));
                                    closing_tags.push(format!("</{}>", tag_name));
                                    nodes_stack.push(nodes);
                                    nodes_stack.push(children.into());
                                    break;
                                }
                            }
                            syn_rsx::NodeName::Block(_) => {
                                todo!()
                            }
                            syn_rsx::NodeName::Punctuated(_) => {
                                todo!()
                            }
                        };
                    }
                    syn_rsx::Node::Attribute(_) => {}
                    syn_rsx::Node::Text(text) => {
                        template.push_str(
                            &String::try_from(&text.value)
                                .expect("This should be stringable!")
                                .to_string(),
                        );
                    }
                    syn_rsx::Node::Comment(_) => {}
                    syn_rsx::Node::Doctype(_) => {}
                    syn_rsx::Node::Block(block) => {
                        let mut block_content = block.value.into();
                        // find state vars
                        let dependencies = {
                            let mut dependencies =
                                FindDependencies::new(scope_vars.clone(), state_vars.clone());
                            dependencies.visit_expr_mut(&mut block_content);
                            dependencies
                        };

                        for state in dependencies.state_dependencies {
                            let ident = Ident::new(
                                &format!("update_test_{}", state.to_string()),
                                Span::mixed_site(),
                            );
                            update_methods.push(parse_quote! {
                                fn #ident() {}
                            });
                        }

                        errors.extend(dependencies.errors);

                        template.push_str("<!>");
                        let text_n = text_methods.len();
                        let ident = Ident::new(&format!("text_{}", text_n), Span::mixed_site());
                        text_methods.push(parse_quote!(
                            fn #ident(&self) -> String {
                                let super::Scope { count } = &self;
                                //::rapyd::ToDomString::to_dom_string(#block_content)
                                crate::ToDomString::to_dom_string(&#block_content)
                            }
                        ));
                    }
                    syn_rsx::Node::Fragment(_) => {}
                }
            }

            if !going_deeper && nodes_is_empty && !closing_tags.is_empty() {
                template.push_str(&closing_tags.pop().expect("There has to be a closing tag!"));
            }
        }
    }
    HtmlMacroOutput {
        template,
        text_methods,
        errors,
        update_methods,
    }
}

struct Test {
    a: i32,
}

impl Test {
    fn b(&self) -> String {
        "".into()
    }
    fn c(&self) -> String {
        Self::b(&self)
    }
}
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////

#[proc_macro_error]
#[proc_macro_attribute]
pub fn component_test(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &item_struct.ident;
    let component_impl: ItemImpl = parse_quote!(
        impl #struct_ident {
            pub const __TEMPLATE: &dyn Template = &Self::render();
        }
    );
    quote!(
        #item_struct
        #component_impl
        impl Lifecycle for Counter {}
    )
    .into()
}

#[proc_macro_error]
#[proc_macro]
pub fn derived(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut closure = parse_macro_input!(item as ExprClosure);
    /*
    let first = closure.inputs.first_mut();
    if first == Some(&mut parse_quote!(&self)) {
        let pat = first.unwrap();
        *pat = parse_quote!(__cx);
    } else {
        panic!("first argument of derived must be &self");
    }
    */
    //
    let mut inputs = Punctuated::<Pat, Comma>::new();
    inputs.push(parse_quote!(__cx));
    inputs.extend(closure.inputs);
    closure.inputs = inputs;

    let mut body = closure.body.clone();
    let mut visitor = ComponentVisitorNew {
        error: None,
        context_members: Default::default(),
    };
    visitor.visit_expr_mut(&mut (*body));
    *closure.body.as_mut() = parse_quote!({
        let __cx: &Self = __cx;
        #body
    });
    // panic!("{:#?}", visitor.context_members);
    quote!(
        __Derived(#closure)
    )
    .into()
}

struct ComponentVisitorNew {
    error: Option<Error>,
    context_members: HashSet<Member>,
}

impl VisitMut for ComponentVisitorNew {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        let self_ident: Ident = Ident::new("self", Span::mixed_site());
        if *i == self_ident {
            *i = parse_quote!(__cx);
        }
    }
    fn visit_expr_field_mut(&mut self, i: &mut syn::ExprField) {
        let self_ident: Ident = Ident::new("self", Span::mixed_site());
        let self_path = ExprPath {
            attrs: vec![],
            qself: None,
            path: self_ident.into(),
        };
        if *i.base == Expr::Path(self_path) {
            self.context_members.insert(i.member.clone());
        }

        for attr in &mut i.attrs {
            self.visit_attribute_mut(attr);
        }
        self.visit_expr_mut(&mut i.base);
        self.visit_member_mut(&mut i.member);
    }
}
