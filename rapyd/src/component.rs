use gloo::console::debug;
use std::{fmt::Display, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Comment, HtmlTemplateElement, Node};

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

    fn render(&self, root: &Node) {
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
                                .expect("TEMPLATE and WALKS are not compatible!");
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
                    Walk::Replace => {
                        debug!("Walk::Text node type assertion starting!");
                        debug_assert_eq!(node.node_type(), 8);
                        debug!("passed");
                        let comment_to_replace: &Comment = node.unchecked_ref();
                        let text_node = text_nodes
                            .next()
                            .expect("TEMPLATE and WALKS are not compatible!");

                        let arr = js_sys::Array::of1(text_node);
                        comment_to_replace.replace_with_with_node(&arr).expect("This should be a logic bug! Maybe Self::TEMPLATE and Self::WALKS aren't compatible,
                                                                            maybe self.text_nodes got initialized wrong, or maybe it's something else c:");
                        node = text_node.clone().into();
                    }

                    Walk::Event(event_type) => {
                        node.add_event_listener_with_callback(
                            event_type,
                            &self.get_scope_base().event_handlers[0]
                                .as_ref()
                                .unchecked_ref(),
                        )
                        .expect("IDK why this fails, but it should be a logic bug!");
                    }
                }
            }
        }
        root.append_child(&template).unwrap();
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
        Context::get_event_handler(scope)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Walk {
    In(usize),
    Out(usize),
    Over(usize),
    Replace,
    Event(&'static str),
}

impl Walk {
    // pub fn can_combine() -> bool

    pub fn combine(optional_one: &mut Option<Self>, another: Self) -> Result<(), ()> {
        match another {
            Walk::In(n) => Self::combine_in(optional_one, n),
            Walk::Out(n) => Self::combine_out(optional_one, n),
            Walk::Over(n) => Self::combine_over(optional_one, n),
            Walk::Replace => Self::combine_replace(optional_one),
            Walk::Event(event_type) => Self::combine_event(optional_one, event_type),
        }
    }

    pub fn combine_event(
        optional_one: &mut Option<Self>,
        event_type: &'static str,
    ) -> Result<(), ()> {
        match optional_one {
            Some(_) => Err(()),
            None => {
                *optional_one = Some(Walk::Event(event_type));
                Ok(())
            }
        }
    }
    pub fn combine_replace(optional_one: &mut Option<Self>) -> Result<(), ()> {
        match optional_one {
            Some(_) => Err(()),
            None => {
                *optional_one = Some(Walk::Replace);
                Ok(())
            }
        }
    }

    pub fn combine_over(optional_one: &mut Option<Self>, over_n: usize) -> Result<(), ()> {
        debug_assert_ne!(over_n, 0);
        match optional_one {
            Some(one) => match one {
                Walk::In(n) => {
                    debug_assert_ne!(*n, 0);
                    Err(())
                }
                Walk::Out(n) => {
                    debug_assert_ne!(*n, 0);
                    Err(())
                }
                Walk::Over(last_n) => {
                    debug_assert_ne!(*last_n, 0);
                    *last_n += over_n;
                    Ok(())
                }
                Walk::Replace => Err(()),
                Walk::Event(_) => Err(()),
            },
            None => {
                *optional_one = Some(Walk::Over(over_n));
                Ok(())
            }
        }
    }

    pub fn combine_out(optional_one: &mut Option<Self>, out_n: usize) -> Result<(), ()> {
        debug_assert_ne!(out_n, 0);
        match optional_one {
            Some(one) => match one {
                Walk::In(n) => {
                    debug_assert_ne!(*n, 0);
                    if *n > out_n {
                        *n -= out_n;
                    } else if out_n > *n {
                        let n = out_n - *n;
                        *one = Walk::Out(n);
                    } else {
                        *optional_one = None;
                    }
                    Ok(())
                }
                Walk::Out(n) => {
                    debug_assert_ne!(*n, 0);
                    *n += 1;
                    Ok(())
                }
                Walk::Over(n) => {
                    debug_assert_ne!(*n, 0);
                    *optional_one = Some(Walk::Out(out_n));
                    Ok(())
                }
                Walk::Replace => Err(()),
                Walk::Event(_) => Err(()),
            },
            None => {
                *optional_one = Some(Walk::Out(out_n));
                Ok(())
            }
        }
    }

    pub fn combine_in(optional_one: &mut Option<Self>, in_n: usize) -> Result<(), ()> {
        debug_assert_ne!(in_n, 0);
        match optional_one {
            Some(one) => match one {
                Walk::In(n) => {
                    debug_assert_ne!(*n, 0);
                    *n += 1;
                    Ok(())
                }
                Walk::Out(n) => {
                    debug_assert_ne!(*n, 0);
                    if *n > in_n {
                        *n -= in_n;
                    } else if in_n > *n {
                        let n = in_n - *n;
                        *one = Walk::In(n);
                    } else {
                        *optional_one = None;
                    }
                    Ok(())
                }
                Walk::Over(n) => {
                    debug_assert_ne!(*n, 0);
                    Err(())
                }
                Walk::Replace => Err(()),
                Walk::Event(_) => Err(()),
            },
            None => {
                *optional_one = Some(Walk::In(in_n));
                Ok(())
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct WalkIterator {
    walks: Vec<Walk>,
}

impl WalkIterator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<WalkIterator> for Vec<Walk> {
    fn from(value: WalkIterator) -> Self {
        value.walks
    }
}

impl WalkIterator {
    pub fn push_replace(&mut self) {
        self.walks.push(Walk::Replace);
    }

    pub fn push_over(&mut self, over_n: usize) {
        debug_assert_ne!(over_n, 0);
        match self.walks.last_mut() {
            Some(walk) => match walk {
                Walk::In(_) => self.walks.push(Walk::Over(over_n)),
                Walk::Out(_) => self.walks.push(Walk::Over(over_n)),
                Walk::Over(last_n) => {
                    debug_assert_ne!(*last_n, 0);
                    *last_n += over_n;
                }
                Walk::Replace => self.walks.push(Walk::Over(over_n)),
                Walk::Event(_) => self.walks.push(Walk::Over(over_n)),
            },
            None => self.walks.push(Walk::Over(over_n)),
        }
    }

    pub fn push_out(&mut self, out_n: usize) {
        debug_assert_ne!(out_n, 0);
        match self.walks.last_mut() {
            Some(walk) => match walk {
                Walk::In(n) => {
                    debug_assert_ne!(*n, 0);
                    if *n > out_n {
                        *n -= out_n;
                    } else if out_n > *n {
                        let n = out_n - *n;
                        self.walks.pop();
                        self.push_out(n);
                    } else {
                        self.walks.pop();
                    }
                }
                Walk::Out(n) => {
                    debug_assert_ne!(*n, 0);
                    *n += out_n;
                }
                Walk::Over(n) => {
                    debug_assert_ne!(*n, 0);
                    self.walks.pop();
                    self.push_out(out_n);
                }
                Walk::Replace => self.walks.push(Walk::Out(out_n)),
                Walk::Event(_) => self.walks.push(Walk::Out(out_n)),
            },
            None => self.walks.push(Walk::Out(out_n)),
        }
    }

    pub fn push_in(&mut self, in_n: usize) {
        debug_assert_ne!(in_n, 0);
        match self.walks.last_mut() {
            Some(walk) => match walk {
                Walk::In(n) => {
                    debug_assert_ne!(*n, 0);
                    *n += 1;
                }
                Walk::Out(n) => {
                    debug_assert_ne!(*n, 0);
                    if *n > in_n {
                        *n -= in_n;
                    } else if in_n > *n {
                        let n = in_n - *n;
                        self.walks.pop();
                        self.push_in(n);
                    } else {
                        self.walks.pop();
                    }
                }
                Walk::Over(n) => {
                    debug_assert_ne!(*n, 0);
                    self.walks.push(Walk::In(in_n))
                }
                Walk::Replace => self.walks.push(Walk::In(in_n)),
                Walk::Event(_) => self.walks.push(Walk::In(in_n)),
            },
            None => self.walks.push(Walk::In(in_n)),
        }
    }
}
