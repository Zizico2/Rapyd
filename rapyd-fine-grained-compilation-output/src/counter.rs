use std::{cell::RefCell, ops::Deref, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    scope::{ScopeState, StateTag, UpdateState},
    Walk,
};

use enclose::enclose;

pub const SCOPED_N_TEXT_NODES: usize = 1;
pub const SCOPED_N_EVENT_TARGETS: usize = 1;

pub const N_TEXT_NODES: usize = SCOPED_N_TEXT_NODES;
pub const N_EVENT_TARGETS: usize = SCOPED_N_EVENT_TARGETS;

pub type TextNodes = [web_sys::Text; SCOPED_N_TEXT_NODES];
pub type EventTargets = [web_sys::EventTarget; SCOPED_N_EVENT_TARGETS];

// STATE -----------------------------------------------------------

/* STATE */

pub struct State0(u32);

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
pub fn new_scope(props: [(); 0], text_nodes: TextNodes, event_targets: EventTargets) -> Rc<Scope> {
    Scope::new(props, text_nodes, event_targets)
}

impl Scope {
    fn setup(
        /*state: Rc<RefCell<State>>, */ scope: Rc<Scope>,
        handle_click: Closure<dyn FnMut()>,
    ) {
        //let handle_click = { Closure::<dyn FnMut()>::new(move || handle_click(state.clone())) };

        scope.event_targets[0]
            .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
            .unwrap();
        //TODO DON'T DO THIS. STORE THIS CLOSURE HANDLE FOR LATER CLEANUP
        handle_click.forget();
    }
    fn new(props: [(); 0], text_nodes: TextNodes, event_targets: EventTargets) -> Rc<Scope> {
        let scope = Rc::new(Scope {
            props: Props,
            text_nodes,
            event_targets,
        });
        // user-code
        let state_0 = ScopeState::new(State0(0), scope.clone());
        //let state_0_1 = state_0.clone();
        let handle_click = enclose!((state_0) move || {
            let mut state_0 = state_0.borrow_mut();
            *state_0  += 1;
        });

        // user-code
        Self::setup(scope.clone(), Closure::new(handle_click));
        state_0.update_state();

        scope
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

mod state {
    struct _0 {}
}
