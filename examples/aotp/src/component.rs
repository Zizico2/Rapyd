use std::{fmt::Display, marker::PhantomData};

pub trait Context {}
pub trait Scope<const N_TEXT_NODES: usize>: Sized {
    type Context: self::Context;
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;
    fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES];
    fn get_context(&self) -> &Self::Context;

    fn update_text_node<const INDEX: usize>(&self)
    where
        self::TextNodeBase<INDEX, N_TEXT_NODES, Self>: TextNode<N_TEXT_NODES, Self>,
    {
        let text_node = self.get_text_nodes();
        let new_data =
            <self::TextNodeBase<INDEX, N_TEXT_NODES, Self>>::get_data(self.get_context());

        let old_data = text_node[INDEX].data();

        if new_data != old_data {
            text_node[INDEX].set_data(new_data.as_str());
        }
    }
}

// TODO this should use #![feature(generic_const_exprs)], and not need a generic param
pub trait TextNode<const N_TEXT_NODES: usize, Scope: self::Scope<N_TEXT_NODES>> {
    fn get_data(cx: &Scope::Context) -> String;
}

pub trait ToData {
    fn to_data(&self) -> String;
}
impl<T: Display> ToData for T {
    fn to_data(&self) -> String {
        self.to_string()
    }
}

pub struct TextNodeBase<
    const INDEX: usize,
    const N_TEXT_NODES: usize,
    Context: self::Scope<N_TEXT_NODES>,
> {
    _marker: PhantomData<Context>,
}

pub struct ScopeBase<const N_TEXT_NODES: usize, Context: self::Context> {
    pub cx: Context,
    pub text_nodes: [web_sys::Text; N_TEXT_NODES],
}
