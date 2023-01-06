use std::{fmt::Display, rc::Rc};

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Context<const N_TEXT_NODES: usize> {
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;

    type Scope: self::Scope<N_TEXT_NODES>;
}

pub trait WithTextNode<const INDEX: usize, const N_TEXT_NODES: usize>:
    Context<N_TEXT_NODES>
{
    fn get_text_node_data(&self) -> String;
}

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Scope<const N_TEXT_NODES: usize>: WithProps {
    type Context: self::Context<N_TEXT_NODES, Scope = Self>;
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;

    //#![feature(associated_type_defaults)]
    type Props;

    fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES];
    fn get_context(&self) -> &Self::Context;

    fn update_text_node<const INDEX: usize>(&self)
    where
        <Self as Scope<N_TEXT_NODES>>::Context: WithTextNode<INDEX, N_TEXT_NODES>,
    {
        let text_node = self.get_text_nodes();
        let new_data = self.get_context().get_text_node_data();

        let old_data = text_node[INDEX].data();

        if new_data != old_data {
            text_node[INDEX].set_data(new_data.as_str());
        }
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

pub struct ScopeBase<const N_TEXT_NODES: usize, Context: self::Context<N_TEXT_NODES, Scope = Self>>
{
    pub cx: Context,
    pub text_nodes: [web_sys::Text; N_TEXT_NODES],
}

impl<const N_TEXT_NODES: usize, Context: self::Context<N_TEXT_NODES, Scope = Self>>
    self::Scope<N_TEXT_NODES> for ScopeBase<N_TEXT_NODES, Context>
where
    Self: WithProps,
{
    type Context = Context;
    type Props = <Self as WithProps>::Props;

    fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES] {
        &self.text_nodes
    }

    fn get_context(&self) -> &Self::Context {
        &self.cx
    }
}
