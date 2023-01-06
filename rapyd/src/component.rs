use gloo::{
    events::{EventListener, EventListenerOptions},
    timers::callback::Timeout,
};
use std::{fmt::Display, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Event;
use web_sys::{Comment, HtmlBodyElement, HtmlElement, HtmlTemplateElement, Node, Window};

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Context<const N_TEXT_NODES: usize, const N_WALKS: usize, const N_EVENT_LISTENERS: usize> {
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;

    type Scope: self::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>;
}

pub trait WithTextNode<
    const INDEX: usize,
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
>: Context<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>
{
    fn get_text_node_data(&self) -> String;
}

pub trait WithEventHandler<
    const INDEX: usize,
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
>: Context<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>
{
    fn get_event_handler(scope: Rc<Self::Scope>) -> Closure<dyn Fn(&web_sys::Event)>;
}

// TODO this should use #![feature(generic_const_exprs)], and should not need a generic param
pub trait Scope<const N_TEXT_NODES: usize, const N_WALKS: usize, const N_EVENT_LISTENERS: usize>:
    WithProps + Sized
{
    type Context: self::Context<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope = Self>;
    // This associated const should never be overriden
    const N_TEXT_NODES: usize = N_TEXT_NODES;
    const N_EVENT_LISTENERS: usize = N_EVENT_LISTENERS;

    const N_WALKS: usize = N_WALKS;
    const TEMPLATE: &'static str;
    const WALKS: [Walk; N_WALKS];

    fn mount(&self) -> web_sys::DocumentFragment {
        let document = gloo_utils::document();

        let template: HtmlTemplateElement = document
            .create_element("template")
            .unwrap()
            .unchecked_into(); // unsafe

        template.set_inner_html(Self::TEMPLATE);

        let template = template.content();

        let node: Option<Node> = template.first_child();
        let scope_base = self.get_scope_base();

        let mut text_nodes = scope_base.text_nodes.iter();

        // TODO could be iter() instead of into_iter(). Performance could be better? could benchmark, eventually
        // let mut walks = Self::WALKS.into_iter().peekable();
        if let Some(mut node) = node {
            for walk in Self::WALKS {
                match walk {
                    Walk::In(n) => {
                        // TODO: performance?
                        for _ in 0..n {
                            node = node
                                .first_child()
                                .expect("TEMPLATE and WALKS are not compatible!")
                        }
                    }
                    Walk::Out(n) => {
                        // TODO: performance?
                        for _ in 0..n {
                            node = node
                                .parent_node()
                                .expect("TEMPLATE and WALKS are not compatible!")
                        }
                    }
                    Walk::Over(n) => {
                        // TODO: performance?
                        for _ in 0..n {
                            node = node
                                .next_sibling()
                                .expect("TEMPLATE and WALKS are not compatible!")
                        }
                    }
                    Walk::Text => {
                        let node: &Comment = node.unchecked_ref();
                        let text_node = text_nodes
                            .next()
                            .expect("TEMPLATE and WALKS are not compatible!");

                        let arr = js_sys::Array::of1(text_node);
                        node.replace_with_with_node(&arr).expect("This should be a logic bug! Maybe Self::TEMPLATE and Self::WALKS aren't compatible,
                                                                            maybe self.text_nodes got initialized wrong, or maybe it's something else c:");
                    }

                    Walk::Event(event_type) => {
                        // TODO: think over EventListenerOptions and `.forget()`
                        EventListener::new_with_options(
                            &node,
                            event_type,
                            EventListenerOptions::default(),
                            |ev| {},
                        )
                        .forget();
                    }
                }
            }
        }
        todo!()
    }

    //#![feature(associated_type_defaults)]
    type Props;

    // fn get_text_nodes(&self) -> &[web_sys::Text; N_TEXT_NODES];
    // fn get_context(&self) -> &Self::Context;
    fn get_scope_base(
        &self,
    ) -> &ScopeBase<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Self::Context, Self>;

    fn update_text_node<const INDEX: usize>(&self)
    where
        Self::Context: WithTextNode<INDEX, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    {
        self.get_scope_base().update_text_node();
    }

    fn get_event_handler<const INDEX: usize>(self: Rc<Self>) -> Closure<dyn Fn(&web_sys::Event)>
    where
        Self::Context: WithEventHandler<INDEX, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    {
        ScopeBase::<
            N_TEXT_NODES,
            N_WALKS,
            N_EVENT_LISTENERS,
            <Self as self::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>>::Context,
            Self,
        >::get_event_handler(self)
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
    const N_EVENT_LISTENERS: usize,
    Context: self::Context<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope = Scope>,
    Scope: self::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Context = Context>,
> {
    pub cx: Context,
    pub text_nodes: [web_sys::Text; N_TEXT_NODES],
    pub event_handlers: [Closure<dyn Fn(&web_sys::Event)>; N_EVENT_LISTENERS],
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        Context: self::Context<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope = Scope>,
        Scope: self::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Context = Context>,
    > ScopeBase<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Context, Scope>
{
    pub const N_TEXT_NODES: usize = N_TEXT_NODES;
    pub const N_EVENT_LISTENERS: usize = N_EVENT_LISTENERS;
    fn update_text_node<const INDEX: usize>(&self)
    where
        Context: WithTextNode<INDEX, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    {
        let new_data = self.cx.get_text_node_data();

        let old_data = self.text_nodes[INDEX].data();

        if new_data != old_data {
            self.text_nodes[INDEX].set_data(new_data.as_str());
        }
    }

    fn get_event_handler<const INDEX: usize>(scope: Rc<Scope>) -> Closure<dyn Fn(&web_sys::Event)>
    where
        Context: WithEventHandler<INDEX, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    {
        scope.get_event_handler()
    }
}

pub enum Walk {
    In(usize),
    Out(usize),
    Over(usize),
    Text,
    Event(&'static str),
}
