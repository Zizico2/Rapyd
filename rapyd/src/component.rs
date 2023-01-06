use std::{fmt::Display, rc::Rc};

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Context<const N_TEXT_NODES: usize, const N_WALKS: usize> {
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;

    type Scope: self::Scope<N_TEXT_NODES, N_WALKS>;
}

pub trait WithTextNode<const INDEX: usize, const N_TEXT_NODES: usize, const N_WALKS: usize>:
    Context<N_TEXT_NODES, N_WALKS>
{
    fn get_text_node_data(&self) -> String;
}

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Scope<const N_TEXT_NODES: usize, const N_WALKS: usize>: WithProps + Sized {
    type Context: self::Context<N_TEXT_NODES, N_WALKS, Scope = Self>;
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;

    const N_WALKS: usize = N_WALKS;
    const TEMPLATE: &'static str;
    const WALKS: [Walk; N_WALKS];

    //#![feature(associated_type_defaults)]
    type Props;

    // fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES];
    // fn get_context(&self) -> &Self::Context;
    fn get_scope_base(&self) -> &ScopeBase<N_TEXT_NODES, N_WALKS, Self::Context, Self>;

    fn update_text_node<const INDEX: usize>(&self)
    where
        Self::Context: WithTextNode<INDEX, N_TEXT_NODES, N_WALKS>,
    {
        self.get_scope_base().update_text_node();
    }
}

pub trait Props: Into<Self::ProcessedProps> {
    type ProcessedProps: ProcessedProps<Props = Self>;

    fn process(self) -> Self::ProcessedProps {
        self.into()
    }
}
pub trait ProcessedProps {
    type Props: Props<ProcessedProps = Self>;
}

pub trait WithProps {
    type Props: Props;
    fn new(props: Self::Props) -> Rc<Self>;
}

pub trait ToData {
    fn to_data(&self) -> String;
}
impl<T: Display> ToData for T {
    fn to_data(&self) -> String {
        self.to_string()
    }
}

pub struct ScopeBase<
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    Context: self::Context<N_TEXT_NODES, N_WALKS, Scope = Scope>,
    Scope: self::Scope<N_TEXT_NODES, N_WALKS, Context = Context>,
> {
    pub cx: Context,
    pub text_nodes: [web_sys::Text; N_TEXT_NODES],
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        Context: self::Context<N_TEXT_NODES, N_WALKS, Scope = Scope>,
        Scope: self::Scope<N_TEXT_NODES, N_WALKS, Context = Context>,
    > ScopeBase<N_TEXT_NODES, N_WALKS, Context, Scope>
{
    pub const N_TEXT_NODES: usize = N_TEXT_NODES;
    fn update_text_node<const INDEX: usize>(&self)
    where
        Context: WithTextNode<INDEX, N_TEXT_NODES, N_WALKS>,
    {
        let new_data = self.cx.get_text_node_data();

        let old_data = self.text_nodes[INDEX].data();

        if new_data != old_data {
            self.text_nodes[INDEX].set_data(new_data.as_str());
        }
    }
}

pub enum Walk {
    In(usize),
    Out(usize),
    Over(usize),
    Text,
    Event(&'static str),
}
