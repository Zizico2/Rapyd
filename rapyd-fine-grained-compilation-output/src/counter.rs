use std::{
    array,
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};
use std::{mem, ptr::NonNull};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::{
    scope::{ScopeState, StateTag, UpdateState},
    Walk,
};

pub const N_TEXT_NODES: usize = 1;
pub const N_EVENT_TARGETS: usize = 1;

pub type TextNodes = [web_sys::Text; N_TEXT_NODES];
pub type EventTargets = [web_sys::EventTarget; N_EVENT_TARGETS];

// STATE -----------------------------------------------------------

/* STATE */

pub struct State0(u32);

impl State0 {
    fn new(val: u32) -> Self {
        State0(val)
    }
}

impl StateTag<u32> for State0 {}

impl From<State0> for u32 {
    fn from(state: State0) -> Self {
        state.0
    }
}

impl Deref for State0 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UpdateState<State0, u32> for Scope {
    fn update_state(&self, new: &u32) {
        self.text_nodes[0].set_data(&new.to_string())
    }
}

/*------------------------------------*/

// ------------------------------------------------------------------
/*
pub mod state {
    pub type Var0 = u32;
}
 */
pub struct Props;

pub struct Scope {
    pub props: Props,
    pub text_nodes: TextNodes,
    pub event_targets: EventTargets,
}
pub fn new_scope<
    'a,
    T: Iterator<Item = &'a web_sys::Text>,
    E: Iterator<Item = &'a web_sys::EventTarget>,
>(
    props: [(); 0],
    mut text_nodes: T,
    mut event_targets: E,
) -> (Rc<Scope>, Rc<RefCell<State>>) {
    let text_nodes: TextNodes = array::from_fn(|_| {
        console::log_1(&"text_next".into());
        text_nodes
            .next()
            .expect("Too few TextNodes for counter")
            .clone()
    });
    let event_targets: EventTargets = array::from_fn(|_| {
        console::log_1(&"event_next".into());
        event_targets
            .next()
            .expect("Too few TextNodes for counter")
            .clone()
    });
    Scope::new(props, text_nodes, event_targets)
}

fn handle_click(state: Rc<RefCell<State>>) {
    let mut state = state.deref().borrow_mut();
    let mut state = state.state_0.borrow_mut();
    *state += 1;
}

impl Scope {
    fn setup(mut state: Rc<RefCell<State>>, scope: Rc<Scope>) {
        std::mem::drop(state.deref().borrow_mut().state_0.borrow_mut());

        let handle_click = { Closure::<dyn FnMut()>::new(move || handle_click(state.clone())) };

        scope.event_targets[0]
            .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
            .unwrap();
        //TODO DON'T DO THIS. STORE THIS CLOSURE HANDLE FOR LATER CLEANUP
        handle_click.forget();
    }
    fn new(
        props: [(); 0],
        text_nodes: TextNodes,
        event_targets: EventTargets,
    ) -> (Rc<Scope>, Rc<RefCell<State>>) {
        let scope = Rc::new(Scope {
            props: Props,
            text_nodes,
            event_targets,
        });
        let state = Rc::new(RefCell::new(State {
            state_0: ScopeState::new(State0(0), scope.clone()),
        }));
        Self::setup(state.clone(), scope.clone());

        (scope, state)
    }
}

pub struct State {
    pub state_0: ScopeState<u32, State0, Scope>,
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
