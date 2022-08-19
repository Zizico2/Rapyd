use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::Walk;

pub type TextNodes = [web_sys::Text; 1];
pub type EventTargets = [web_sys::EventTarget; 1];

pub mod state {
    pub type Count = u32;
}

#[derive(Clone)]
pub struct Scope {
    state: State,
    text_nodes: TextNodes,
    event_targets: EventTargets,
    //event_handlers
}
fn apply_count(mut scope: RefMut<Scope>, count: state::Count) {
    if scope.state.count != count {
        scope.state.count = count;
        scope.text_nodes[0].set_data(&count.to_string());
    }
}

fn handle_click(scope: RefMut<Scope>) {
    let new_count = scope.state.count + 1;
    apply_count(scope, new_count);
}

impl Scope {
    fn setup(scope: Rc<RefCell<Self>>) {
        let cloned_scope = scope.clone();

        let handle_click =
            Closure::<dyn Fn()>::new(move || handle_click(cloned_scope.borrow_mut()));
        scope.borrow_mut().event_targets[0]
            .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
            .unwrap();
    }
    pub fn new(text_nodes: TextNodes, event_targets: EventTargets) -> Rc<RefCell<Self>> {
        let scope = Rc::new(RefCell::new(Self {
            state: State::new(),
            text_nodes,
            event_targets,
        }));

        Self::setup(scope.clone());

        scope
    }
}

#[derive(Default)]
pub struct NewScopeOptions {
    pub text_nodes: OptionalTextNodes,
    pub event_targets: OptionalEventTargets,
}

pub type OptionalTextNodes = [Option<web_sys::Text>; 1];
pub type OptionalEventTargets = [Option<web_sys::EventTarget>; 1];

#[derive(Clone, Copy)]
pub struct State {
    count: state::Count,
}

impl State {
    fn new() -> Self {
        Self { count: 0 }
    }
}

pub const TEMPLATE: &str = "<button>clicks: <!></button>";

pub type Walks = [Walk; 4];

pub const WALKS: Walks = [
    Walk::Next(1, true),
    Walk::Over(1),
    Walk::Replace,
    Walk::Out(1),
];

pub const N_MORE_WALKS: usize = 0;
