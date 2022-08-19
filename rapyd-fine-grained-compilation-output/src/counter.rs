use std::{array, borrow::BorrowMut, cell::RefCell, rc::Rc, slice};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::Walk;

pub const N_TEXT_NODES: usize = 1;
pub const N_EVENT_TARGETS: usize = 1;

pub type TextNodes = [web_sys::Text; N_TEXT_NODES];
pub type EventTargets = [web_sys::EventTarget; N_EVENT_TARGETS];

pub mod state {
    pub type Count = u32;
}

mod inner {
    use std::cell::RefCell;

    use super::{EventTargets, Props, State, TextNodes};

    pub struct Scope {
        pub props: Props,
        pub state: RefCell<State>,
        pub text_nodes: TextNodes,
        pub event_targets: EventTargets,
    }
}

pub type Scope = Rc<inner::Scope>;

pub fn new_scope<'a>(
    text_nodes: &mut slice::IterMut<'a, web_sys::Text>,
    event_targets: &mut slice::IterMut<'a, web_sys::EventTarget>,
) -> Scope {
    let text_nodes = array::from_fn(|_| {
        text_nodes
            .next()
            .expect("Too few TextNodes for counter")
            .clone()
    });
    let event_targets = array::from_fn(|_| {
        event_targets
            .next()
            .expect("Too few TextNodes for counter")
            .clone()
    });

    inner::Scope::new(text_nodes, event_targets)
}

fn apply_count(scope: Scope, count: state::Count) {
    let mut state = scope.state.borrow_mut();
    if state.count != count {
        state.count = count;
        std::mem::drop(state);
        update_count(scope);
    }
}
pub struct Props;

// update every node dependant on count
// this shouldn't be RefMut. This could very wel be a Ref
//TODO Think about this
fn update_count(scope: Scope) {
    let state = scope.state.borrow();
    scope.text_nodes[0].set_data(&state.count.to_string());
}

fn handle_click(scope: Scope) {
    let state = scope.state.borrow_mut();
    let new_count = state.count + 1;
    std::mem::drop(state);
    apply_count(scope, new_count);
}

impl<'a> inner::Scope {
    fn setup(mut scope: Scope) {
        let handle_click = {
            let scope = scope.clone();
            Closure::<dyn Fn()>::new(move || handle_click(scope.clone()))
        };
        scope.borrow_mut().event_targets[0]
            .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
            .unwrap();
        //TODO DON'T DO THIS. STORE THIS CLOSURE HANDLE FOR LATER CLEANUP
        handle_click.forget();

        update_count(scope);
    }
    fn new(text_nodes: TextNodes, event_targets: EventTargets) -> Scope {
        let scope = Self {
            props: Props,
            state: RefCell::new(State::new()),
            text_nodes,
            event_targets,
        };
        let scope = Rc::new(scope);
        Self::setup(scope.clone());
        scope
    }
}

//#[derive(Default)]
pub struct NewScopeOptions<'a> {
    pub text_nodes: slice::Iter<'a, web_sys::Text>,
    pub event_targets: slice::Iter<'a, web_sys::EventTarget>,
}

//pub type OptionalTextNodes = ArrayVec<Option<web_sys::Text>, 1>;
//pub type OptionalEventTargets = ArrayVec<Option<web_sys::EventTarget>, 1>;

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

pub type Walks = [Walk; 5];

pub const WALKS: Walks = [
    Walk::EventTarget,
    Walk::Next(1),
    Walk::Over(1),
    Walk::Replace,
    Walk::Out(1),
];
