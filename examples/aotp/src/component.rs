use std::{marker::PhantomData, rc::Rc};

//TODO this should use #![feature(generic_const_exprs)], and not need a generic param
// for now it requires the user keeps the consts and the generic in sync by themselves
pub trait Context<const N_TEXT_NODES: usize> {
    const N_TEXT_NODES: usize;
    fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES];
}

pub trait TextNodeToData<Context> {
    fn to_data(cx: Rc<Context>) -> String;
}

pub trait ToData {
    fn to_data(&self) -> String;
}

pub struct TextNode<
    const INDEX: usize,
    const N_TEXT_NODES: usize,
    Context: self::Context<N_TEXT_NODES>,
> {
    _marker: PhantomData<Context>,
}

pub fn update_text_node<
    const INDEX: usize,
    const N_TEXT_NODES: usize,
    Context: self::Context<N_TEXT_NODES>,
>(
    cx: Rc<Context>,
) where
    self::TextNode<INDEX, N_TEXT_NODES, Context>: self::TextNodeToData<Context>,
{
    cx.get_text_nodes()[INDEX].set_data(&<self::TextNode<INDEX, N_TEXT_NODES, Context>>::to_data(
        cx.clone(),
    ));
}
