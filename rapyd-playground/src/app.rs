use std::{cell::RefCell, rc::Rc, slice};

use crate::counter;
use const_format::formatcp;
use web_sys::{window, EventTarget, Node, Text};

use crate::Walk;

pub struct Scope {
    // 2 scopes just so this can be a tuple
    pub child_scopes: (
        Option<Rc<RefCell<counter::Scope>>>,
        Option<Rc<RefCell<counter::Scope>>>,
    ),
}

pub const TEMPLATE: &str = formatcp!("<main>{}</main>", counter::TEMPLATE);

pub type Walks = [Walk; 3];

pub const N_MORE_WALKS: usize = 1 + counter::N_MORE_WALKS;

pub const WALKS: [Walk; 3] = [
    Walk::Next(1, false),
    Walk::MoreWalks(&counter::WALKS),
    Walk::Out(1),
];

pub fn mount(anchor: Node) {
    let document = window()
        .expect("Where's my window")
        .document()
        .expect("Where's my document");
    let fragment = document
        .create_range()
        .expect("Can't create range")
        .create_contextual_fragment(TEMPLATE)
        .expect("Can't contextual fragment");
    let scope = walk_through(fragment.first_child().unwrap(), [Some(WALKS.iter()), None]);

    anchor
        .append_child(&fragment.first_child().unwrap())
        .unwrap();

    std::mem::forget(scope);
}

fn walk_through<const N: usize>(node: Node, walks: [Option<slice::Iter<Walk>>; N]) -> Scope {
    let mut index = 0;
    let mut walks_matrix: [Option<slice::Iter<Walk>>; N] = walks;

    let mut current_node = node;

    let mut more_walks = None;

    let mut counter_scope = counter::NewScopeOptions::default();

    loop {
        let mut text_nodes_counter = 0;
        let mut event_targets_counter = 0;
        let walks = {
            let mut outter_walks = None;
            for walks in walks_matrix.iter_mut().rev() {
                if let Some(walks) = walks {
                    if walks.len() != 0 {
                        outter_walks = Some(walks);
                    }
                }
            }
            match outter_walks {
                Some(walks) => walks,
                None => break,
            }
        };

        for walk in walks {
            match walk {
                Walk::Next(n, _) => {
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

                    current_node = current_node.parent_node().expect("No more parent nodes!");
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

                    counter_scope.text_nodes[text_nodes_counter] = Some(new_text_node.clone());
                    text_nodes_counter += 1;

                    current_node = new_text_node.into();
                }
                Walk::EventTarget => {
                    let et: EventTarget = current_node.clone().into();
                    counter_scope.event_targets[event_targets_counter] = Some(et);
                    event_targets_counter += 1;
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

    Scope {
        child_scopes: (
            Some(counter::Scope::new(
                counter_scope
                    .text_nodes
                    .map(|elem| elem.expect("Missing text node!")),
                counter_scope
                    .event_targets
                    .map(|elem| elem.expect("Missing event target!")),
            )),
            None,
        ),
    }
}
