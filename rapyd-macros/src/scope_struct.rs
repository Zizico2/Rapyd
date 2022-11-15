use std::collections::{HashMap, HashSet};
use std::iter::StepBy;
use std::mem;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::VisitMut;
use syn::{parse_quote, Attribute, Error, Field, FieldsNamed, ItemStruct, Lifetime, Path};

#[derive(Debug)]
pub struct Scope {
    options: ScopeOptions,
    state_fields: HashSet<Ident>,
    prop_fields: HashSet<Ident>,
}

pub struct State {}

impl Parse for State {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

pub struct PropOptions {}
impl Parse for PropOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

impl Default for PropOptions {
    fn default() -> Self {
        Self {}
    }
}

struct ScopeFactory {
    scope_lifetime: Lifetime,
    scope_name: Ident,
}

#[derive(Debug, Hash)]
struct StateField {}

#[derive(Debug, Hash)]
struct PropField {}

struct ScopeField {
    prop: Option<PropField>,
    state: Option<StateField>,
}

impl ScopeFactory {
    fn process_field(&mut self, mut field: Field) -> (Field, ScopeField) {
        /*
            Transform this attribute into a real attribute that handles the transformation.
            Only do here what needs to be done globally, and can't be done by the real attribute.
        */
        let (state_prop_attrs, attrs): (Vec<_>, Vec<_>) = field
            .attrs
            .into_iter()
            .partition(|attr| attr.path == parse_quote!(scope) || attr.path == parse_quote!(state));
        field.attrs = attrs;

        todo!()
    }

    pub fn process_scope(&self, scope: &mut ItemStruct) -> Result<Scope, ScopeError> {
        let scope_options: Result<ScopeOptions, ScopeAttributeError> = {
            let mut attrs = vec![];
            let mut scope_options_vec = vec![];
            let mut error: Option<Error> = None;
            let mut scope_attrs = TokenStream::new();

            while let Some(attr) = scope.attrs.pop() {
                if attr.path == parse_quote!(scope) {
                    match attr.parse_args::<ScopeOptions>() {
                        Ok(scope_options) => scope_options_vec.push(scope_options),
                        Err(new_error) => match &mut error {
                            Some(error) => error.combine(new_error),
                            None => error = Some(new_error),
                        },
                    }
                    attr.to_tokens(&mut scope_attrs);
                } else {
                    attrs.push(attr);
                }
            }
            scope.attrs = attrs;
            match scope_options_vec.len() {
                0 => return Err(ScopeError::NotAScopeError),
                1 => for field in scope.fields.iter_mut() {},
                _ => {
                    let new_error =
                        Error::new_spanned(scope_attrs, "Found multiple scope attributes!");
                    match &mut error {
                        Some(error) => error.combine(new_error),
                        None => error = Some(new_error),
                    }
                }
            }

            // TODO
            match error {
                Some(error) => {
                    return Err(ScopeError::ParsingError(
                        Scope {
                            options: scope_options_vec.pop().unwrap(),
                            state_fields: HashSet::new(),
                            prop_fields: HashSet::new(),
                        },
                        error,
                    ))
                }
                None => {
                    return Ok(Scope {
                        options: scope_options_vec.pop().unwrap(),
                        state_fields: HashSet::new(),
                        prop_fields: HashSet::new(),
                    })
                }
            }
        };
    }
}

enum ScopeAttributeError {
    ParsingError(Vec<ScopeOptions>, Error),
}

enum ScopeError {
    NotAScopeError,
    ParsingError(Scope, Error),
}

fn process_scope_fields(fields: &mut FieldsNamed) {}

#[derive(Debug)]
pub struct ScopeOptions {}

impl Parse for ScopeOptions {
    // this sgould be called by parse_args
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ScopeOptions {})
    }
}

impl Default for ScopeOptions {
    fn default() -> Self {
        Self {}
    }
}
