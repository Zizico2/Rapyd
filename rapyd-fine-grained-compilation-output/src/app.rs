use std::slice;

use crate::counter;
use arrayvec::ArrayVec;
use const_format::formatcp;
use web_sys::{console, Element, EventTarget, Node, Text};

use crate::Walk;

enum NewScopeValue {
    Text(web_sys::Text),
    EventTarget(web_sys::EventTarget),
}

impl TryFrom<NewScopeValue> for web_sys::Text {
    type Error = (); //TODO

    fn try_from(value: NewScopeValue) -> Result<Self, Self::Error> {
        match value {
            NewScopeValue::Text(text) => Ok(text),
            _ => Err(()),
        }
    }
}
impl TryFrom<NewScopeValue> for web_sys::EventTarget {
    type Error = (); //TODO

    fn try_from(value: NewScopeValue) -> Result<Self, Self::Error> {
        match value {
            NewScopeValue::EventTarget(event_target) => Ok(event_target),
            _ => Err(()),
        }
    }
}

impl From<web_sys::Text> for NewScopeValue {
    fn from(text: web_sys::Text) -> Self {
        Self::Text(text)
    }
}

impl From<web_sys::EventTarget> for NewScopeValue {
    fn from(text: web_sys::EventTarget) -> Self {
        Self::EventTarget(text)
    }
}

pub struct Scope {
    pub child_scopes: ChildScopes,
}

pub struct ChildScopes(counter::Scope, counter::Scope);

pub const TEMPLATE: &str = formatcp!("<main>{}{}</main>", counter::TEMPLATE, counter::TEMPLATE);

pub type Walks = [Walk; 4];

pub const WALKS: Walks = [
    Walk::Next(1),
    Walk::ChildWalks(&counter::WALKS),
    Walk::ChildWalks(&counter::WALKS),
    Walk::Out(1),
];

pub const N_TEXT_NODES: usize = 0;
pub const N_EVENT_TARGETS: usize = 0;

pub fn mount(anchor: Element) {
    anchor.set_inner_html(TEMPLATE);
    let mut array = ArrayVec::<slice::Iter<Walk>, 3>::new_const();

    array.push(WALKS.iter());
    const AUX_N_TEXT_NODES: usize = counter::N_TEXT_NODES + counter::N_TEXT_NODES + N_TEXT_NODES;
    const AUX_N_EVENT_TARGETS: usize =
        counter::N_EVENT_TARGETS + counter::N_EVENT_TARGETS + N_EVENT_TARGETS;
    let (mut text_nodes, mut event_targets) = walk_through::<
        3,
        AUX_N_TEXT_NODES,
        AUX_N_EVENT_TARGETS,
    >(anchor.first_child().unwrap(), array);

    let mut text_nodes = text_nodes.iter_mut();
    let mut event_targets = event_targets.iter_mut();
    let scope = Scope {
        child_scopes: ChildScopes(
            counter::new_scope(&mut text_nodes, &mut event_targets),
            counter::new_scope(&mut text_nodes, &mut event_targets),
        ),
    };
    std::mem::forget(scope);
}

fn walk_through<const N_WALKS: usize, const N_TEXT_NODES: usize, const N_EVENT_TARGETS: usize>(
    node: Node,
    walks: ArrayVec<slice::Iter<Walk>, N_WALKS>,
) -> (
    ArrayVec<web_sys::Text, N_TEXT_NODES>,
    ArrayVec<web_sys::EventTarget, N_EVENT_TARGETS>,
) {
    let mut text_nodes = ArrayVec::<web_sys::Text, N_TEXT_NODES>::new_const();
    let mut event_targets = ArrayVec::<web_sys::EventTarget, N_EVENT_TARGETS>::new_const();

    let mut walks_matrix = walks;

    let mut current_node = node;

    let mut more_walks = None;

    loop {
        let walks = {
            let mut outter_walks = None;
            for walks in walks_matrix.as_mut_slice().iter_mut().rev() {
                if walks.len() != 0 {
                    outter_walks = Some(walks);
                    break;
                }
            }
            match outter_walks {
                Some(outter_walks) => outter_walks,
                None => break,
            }
        };

        for walk in walks {
            match walk {
                Walk::Next(n) => {
                    console::log_1(&"Next".into());
                    for _ in 0..n - 1 {
                        current_node.first_child().expect("No more child nodes!");
                    }

                    current_node = current_node.first_child().expect("No more child nodes!");
                }
                Walk::Over(n) => {
                    console::log_1(&"Over".into());
                    for _ in 0..n - 1 {
                        current_node
                            .next_sibling()
                            .expect("No more next_sibling nodes!");
                    }

                    current_node = current_node
                        .next_sibling()
                        .expect("No more next_sibling nodes!");
                }
                Walk::Out(n) => {
                    console::log_1(&"Out".into());
                    for _ in 0..n - 1 {
                        current_node.parent_node().expect("No more parent nodes!");
                    }

                    let node = current_node
                        .parent_node()
                        .expect("No more parent nodes!")
                        .next_sibling();

                    match node {
                        Some(node) => current_node = node,
                        None => break,
                    }
                }
                Walk::Replace => {
                    console::log_1(&"Replace".into());
                    let new_text_node = Text::new().unwrap();
                    current_node
                        // could be optimized, so parent_node isn't queried every time. Could cache it locally.
                        // higher memory usage vs better performance
                        // check if performance is actually better
                        .parent_node()
                        .unwrap()
                        .replace_child(&new_text_node, &current_node)
                        .unwrap();

                    //map_new_scope(new_text_node.clone().into());
                    text_nodes.push(new_text_node.clone().into());

                    current_node = new_text_node.into();
                }
                Walk::EventTarget => {
                    console::log_1(&"EventTarget".into());
                    let et: EventTarget = current_node.clone().into();
                    event_targets.push(et.into());
                    //map_new_scope(et.into());
                }
                Walk::ChildWalks(walks) => {
                    more_walks = Some(walks.iter());
                    break;
                }
            }
        }
        if let Some(inner_more_walks) = more_walks {
            walks_matrix.push(inner_more_walks);
            more_walks = None;
        }
    }

    (text_nodes, event_targets)
}
