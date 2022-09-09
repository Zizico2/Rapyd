use std::{ops::AddAssign, rc::Rc};

use proc_macro2::TokenStream;
use web_sys::EventTarget;
use web_sys::Text;

pub mod state;
pub mod util;

//rule of thumb: variants that take a usize should be consuming, the others shouldn't
#[derive(Clone, Debug, Copy)]
pub enum Walk {
    // go n levels deeper
    Next(usize),
    // skip n nodes
    Over(usize),
    // go n levels shallower
    Out(usize),
    // replace the next node. Doesn't move forward.
    // if you were to do Replace again, you would replace the newly inserted node
    //
    // if you were to do EventTarget after Replace you would be flagging
    // the newly inserted node
    Replace,
    // flag the next node as an event target. Doesn't move forward.
    EventTarget,
}

pub struct WalkFactory {
    walks: Vec<Walk>,
}

impl WalkFactory {
    pub fn push(&mut self, mut new_walk: Walk) {
        let last = match self.walks.last_mut() {
            Some(last) => last,
            None => {
                self.walks.push(new_walk);
                return;
            }
        };

        //TODO this should be able to be optimized further (minimize the number of walks)
        match (last, &mut new_walk) {
            (Walk::Next(n), Walk::Next(new_n)) => {
                *n += *new_n;
            }
            (Walk::Out(n), Walk::Out(new_n)) => {
                *n += *new_n;
            }
            (Walk::Over(n), Walk::Over(new_n)) => {
                *n += *new_n;
            }

            (Walk::Next(n), Walk::Out(new_n)) => {
                if n > new_n {
                    *n -= *new_n;
                } else if n < new_n {
                    let n = *new_n - *n;
                    self.walks.pop();
                    self.push(Walk::Out(n));
                } else {
                    self.walks.pop();
                }
            }
            (Walk::Out(n), Walk::Next(new_n)) => {
                if n > new_n {
                    *n -= *new_n;
                } else if n < new_n {
                    let n = *new_n - *n;
                    self.walks.pop();
                    self.push(Walk::Next(n));
                } else {
                    self.walks.pop();
                }
            }
            (Walk::Over(_), Walk::Out(_)) => {
                self.walks.pop();
                self.walks.push(new_walk);
            }
            _ => self.walks.push(new_walk),
        }
    }
}

#[derive(Debug)]
pub struct Dom<const T: usize, const E: usize> {
    pub text_nodes: [Text; T],
    pub event_targets: [EventTarget; E],
}

#[derive(Debug)]
pub struct Scope<
    const SCOPED_N_TEXT_NODES: usize,
    const SCOPED_N_EVENT_TARGETS: usize,
    Props,
    ChildScopes,
> {
    pub props: Props,
    pub dom: Dom<SCOPED_N_TEXT_NODES, SCOPED_N_EVENT_TARGETS>,
    pub child_scopes: ChildScopes,
}

impl<const N_TEXT_NODES: usize, const N_EVENT_TARGETS: usize, Props, ChildScopes>
    Scope<N_TEXT_NODES, N_EVENT_TARGETS, Props, ChildScopes>
{
    pub fn new(
        props: Props,
        text_nodes: [Text; N_TEXT_NODES],
        event_targets: [EventTarget; N_EVENT_TARGETS],
        child_scopes: ChildScopes,
    ) -> Rc<Self> {
        Rc::new(Self {
            props,
            dom: Dom {
                text_nodes,
                event_targets,
            },
            child_scopes,
        })
    }
}
