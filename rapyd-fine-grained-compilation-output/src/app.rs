use std::{cell::RefCell, rc::Rc, slice};

use crate::{counter, scope::split_array};
use array_concat::*;
use arrayvec::ArrayVec;
use const_format::concatcp;
use web_sys::{console, Element, EventTarget, Node, Text};

use crate::Walk;

mod before_compilation {
    mod counter {
        use rapyd_macros::mock_component;
        //pub const TEMPLATE: &str = "<button>clicks: <!></button>";
        mock_component! {
                <script>
                    #[state]
                    let count = 0;

                    let handle_click = |_| {
                            let count = count.borrow_mut():
                            *count += 1;
                    }

                </script>
                <template>
                    <button on:click={handle_click}>
                        "clicks: " { count }
                    </button>
                </template>
        }
    }

    mod app {
        use rapyd_macros::mock_component;
        mock_component! {
                <template>
                    <Counter>
                    </Counter>
                    <Counter>
                    </Counter>
                </template>
        }
    }
}

pub struct Scope {
    pub child_scopes: ChildScopes,
}

pub struct ChildScopes(Rc<counter::Scope>, Rc<counter::Scope>, Rc<counter::Scope>);

pub const TEMPLATE: &str = concatcp!(
    "<main>",
    counter::TEMPLATE,
    counter::TEMPLATE,
    counter::TEMPLATE,
    "</main>"
);

const WALK_PARTS_0: [Walk; 1] = [Walk::Next(1)];
const WALK_PARTS_1: counter::Walks = counter::WALKS;
const WALK_PARTS_2: counter::Walks = counter::WALKS;
const WALK_PARTS_3: counter::Walks = counter::WALKS;
const WALK_PARTS_4: [Walk; 1] = [Walk::Out(1)];

pub type Walks = [Walk;
    concat_arrays_size!(
        WALK_PARTS_0,
        WALK_PARTS_1,
        WALK_PARTS_2,
        WALK_PARTS_3,
        WALK_PARTS_4
    )];

pub const WALKS: Walks = concat_arrays!(
    WALK_PARTS_0,
    WALK_PARTS_1,
    WALK_PARTS_2,
    WALK_PARTS_3,
    WALK_PARTS_4
);

// The number of text nodes created by this component
pub const SCOPED_TEXT_NODES: usize = 0;

// The number of event targets created by this component
pub const SCOPED_N_EVENT_TARGETS: usize = 0;

// The number of text nodes created by this component and all of its children
const N_TEXT_NODES: usize =
    SCOPED_TEXT_NODES + counter::N_TEXT_NODES + counter::N_TEXT_NODES + counter::N_TEXT_NODES;

// The number of event targets created by this component and all of its children
const N_EVENT_TARGETS: usize = SCOPED_N_EVENT_TARGETS
    + counter::N_EVENT_TARGETS
    + counter::N_EVENT_TARGETS
    + counter::N_EVENT_TARGETS;

pub fn mount(anchor: Element) {
    anchor.set_inner_html(TEMPLATE);

    let (_text_nodes, _event_targets) = walk_through::<3, N_TEXT_NODES, N_EVENT_TARGETS>(
        anchor.first_child().unwrap(),
        WALKS.iter(),
    );

    // Compile-time generated

    const _REMAINING_TEXT_NODES_0: usize = N_TEXT_NODES - counter::SCOPED_N_TEXT_NODES;
    const _REMAINING_EVENT_TARGETS_0: usize = N_EVENT_TARGETS - counter::SCOPED_N_EVENT_TARGETS;
    let (scoped_text_nodes, _text_nodes): (
        [Text; counter::SCOPED_N_TEXT_NODES],
        [Text; _REMAINING_TEXT_NODES_0],
    ) = split_array(_text_nodes);
    let (scoped_event_targets, _event_targets): (
        [EventTarget; counter::SCOPED_N_TEXT_NODES],
        [EventTarget; _REMAINING_TEXT_NODES_0],
    ) = split_array(_event_targets);
    let scope_0 = counter::new_scope([], scoped_text_nodes, scoped_event_targets);

    //------------------------------------------

    const _REMAINING_TEXT_NODES_1: usize = _REMAINING_TEXT_NODES_0 - counter::SCOPED_N_TEXT_NODES;
    const _REMAINING_EVENT_TARGETS_1: usize =
        _REMAINING_EVENT_TARGETS_0 - counter::SCOPED_N_EVENT_TARGETS;
    let (scoped_text_nodes, _text_nodes): (
        [Text; counter::SCOPED_N_TEXT_NODES],
        [Text; _REMAINING_TEXT_NODES_1],
    ) = split_array(_text_nodes);
    let (scoped_event_targets, _event_targets): (
        [EventTarget; counter::SCOPED_N_TEXT_NODES],
        [EventTarget; _REMAINING_TEXT_NODES_1],
    ) = split_array(_event_targets);
    let scope_1 = counter::new_scope([], scoped_text_nodes, scoped_event_targets);

    //------------------------------------------

    const _REMAINING_TEXT_NODES_2: usize = _REMAINING_TEXT_NODES_1 - counter::SCOPED_N_TEXT_NODES;
    const _REMAINING_EVENT_TARGETS_2: usize =
        _REMAINING_EVENT_TARGETS_1 - counter::SCOPED_N_EVENT_TARGETS;
    let (scoped_text_nodes, _text_nodes): (
        [Text; counter::SCOPED_N_TEXT_NODES],
        [Text; _REMAINING_TEXT_NODES_2],
    ) = split_array(_text_nodes);
    let (scoped_event_targets, _event_targets): (
        [EventTarget; counter::SCOPED_N_TEXT_NODES],
        [EventTarget; _REMAINING_TEXT_NODES_2],
    ) = split_array(_event_targets);
    let scope_2 = counter::new_scope([], scoped_text_nodes, scoped_event_targets);

    //------------------------------------------

    let scope = Scope {
        child_scopes: ChildScopes(scope_0, scope_1, scope_2),
    };
    std::mem::forget(scope);
}

fn walk_through<const N_WALKS: usize, const N_TEXT_NODES: usize, const N_EVENT_TARGETS: usize>(
    node: Node,
    walks: slice::Iter<Walk>,
) -> (
    [web_sys::Text; N_TEXT_NODES],
    [web_sys::EventTarget; N_EVENT_TARGETS],
) {
    let mut text_nodes = ArrayVec::<web_sys::Text, N_TEXT_NODES>::new_const();
    let mut event_targets = ArrayVec::<web_sys::EventTarget, N_EVENT_TARGETS>::new_const();

    let mut current_node = node;

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
                let new_text_node = Text::new().unwrap();
                current_node
                    // could be optimized, so parent_node isn't queried every time. Could cache it locally.
                    // higher memory usage vs better performance
                    // check if performance is actually better
                    .parent_node()
                    .unwrap()
                    .replace_child(&new_text_node, &current_node)
                    .unwrap();

                text_nodes.push(new_text_node.clone().into());

                current_node = new_text_node.into();
            }
            Walk::EventTarget => {
                let et: EventTarget = current_node.clone().into();
                event_targets.push(et.into());
            }
        }
    }

    (
        text_nodes.into_inner().expect("Text nodes array not full"),
        event_targets
            .into_inner()
            .expect("Event targets array not full"),
    )
}
