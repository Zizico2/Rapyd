use std::slice;

use crate::counter;
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
    Walk::MoreWalks(&counter::WALKS),
    Walk::MoreWalks(&counter::WALKS),
    Walk::Out(1),
];

pub fn mount(anchor: Element) {
    anchor.set_inner_html(TEMPLATE);
    let _scope = walk_through(
        anchor.first_child().unwrap(),
        [Some(WALKS.iter()), None, None],
    );
    std::mem::forget(_scope);
}

fn walk_through<const N: usize>(node: Node, walks: [Option<slice::Iter<Walk>>; N]) -> Scope {
    let mut index = 0;
    let mut walks_matrix: [Option<slice::Iter<Walk>>; N] = walks;

    let mut current_node = node;

    let mut more_walks = None;

    let mut child_new_scope_options = (
        counter::NewScopeOptions::default(),
        counter::NewScopeOptions::default(),
    );

    let mut scope_counter: u32 = 0;

    let mut map_new_scope = |val: NewScopeValue| {
        match scope_counter {
            0 => child_new_scope_options.0.event_targets[0] = Some(val.try_into().unwrap()),
            1 => child_new_scope_options.0.text_nodes[0] = Some(val.try_into().unwrap()),
            2 => child_new_scope_options.1.event_targets[0] = Some(val.try_into().unwrap()),
            3 => child_new_scope_options.1.text_nodes[0] = Some(val.try_into().unwrap()),
            _ => panic!(),
        };
        scope_counter += 1;
    };

    loop {
        let walks = {
            let mut outter_walks = None;
            for walks in walks_matrix.iter_mut().rev() {
                if let Some(walks) = walks {
                    if walks.len() != 0 {
                        outter_walks = Some(walks);
                        break;
                    }
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
                    for _ in 0..n - 1 {
                        current_node.first_child().expect("No more child nodes!");
                    }

                    current_node = current_node.first_child().expect("No more child nodes!");
                }
                Walk::Over(n) => {
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
                    let new_text_node = Text::new().unwrap();
                    current_node
                        // could be optimized, so parent_node isn't queried every time. Could cache it locally.
                        // higher memory usage vs better performance
                        // check if performance is actually better
                        .parent_node()
                        .unwrap()
                        .replace_child(&new_text_node, &current_node)
                        .unwrap();

                    map_new_scope(new_text_node.clone().into());

                    current_node = new_text_node.into();
                }
                Walk::EventTarget => {
                    let et: EventTarget = current_node.clone().into();
                    map_new_scope(et.into());
                }
                Walk::MoreWalks(walks) => {
                    more_walks = Some(walks.iter());
                    break;
                }
            }
        }
        if more_walks.is_some() {
            index += 1;
            walks_matrix[index] = more_walks;
            more_walks = None;
        }
    }

    let child_scopes = ChildScopes(
        counter::new_scope(
            child_new_scope_options
                .0
                .text_nodes
                .map(|elem| elem.expect("Missing text node!")),
            child_new_scope_options
                .0
                .event_targets
                .map(|elem| elem.expect("Missing event target!")),
        ),
        counter::new_scope(
            child_new_scope_options
                .1
                .text_nodes
                .map(|elem| elem.expect("Missing text node!")),
            child_new_scope_options
                .1
                .event_targets
                .map(|elem| elem.expect("Missing event target!")),
        ),
    );

    Scope { child_scopes }
}
