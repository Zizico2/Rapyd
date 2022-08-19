use bytecheck::CheckBytes;
use const_format::formatcp;
use rkyv::{with, Archive, Deserialize, Serialize};

use crate::Walk;

mod Type {
    pub type A = i32;
    pub type B = i32;
}

type TextNodes = [web_sys::Text; 3];

pub struct Scope {
    props: Props,
    state: State,
    text_nodes: [web_sys::Text; 3],
}
impl Scope {
    fn new(props: Props, text_nodes: [web_sys::Text; 3]) -> Self {
        Self {
            props,
            state: State::new(),
            text_nodes,
        }
    }
}

pub struct State;

impl State {
    fn new() -> Self {
        Self
    }
}

pub struct Props {
    pub a: Type::A, // depends on user specified type
    pub b: Type::B, // depends on user specified type
}


pub const TEMPLATE: &str = "<div><!> + <!> = <!></div>";

pub const WALKS: [Walk; 7] = [
    Walk::Next(1, false),
    Walk::Replace,
    Walk::Over(2),
    Walk::Replace,
    Walk::Over(2),
    Walk::Replace,
    Walk::Out(1),
];

pub const CHILD_WALKS: [Walk; 2] = [Walk::Next(1, false), Walk::Replace];

pub fn apply_a<'a>(scope: &'a mut Scope, a: Type::A) {
    if scope.props.a != a {
        scope.props.a = a;
        scope.text_nodes[0].set_data(&a.to_string());
        applyWith_a_b(scope);
    }
}
pub fn apply_b<'a>(scope: &'a mut Scope, b: Type::B) {
    if scope.props.b != b {
        scope.props.b = b;
        scope.text_nodes[1].set_data(&b.to_string());
        applyWith_a_b(scope);
    }
}

fn applyWith_a_b<'a>(scope: &'a mut Scope) {
    scope.text_nodes[2].set_data(&(scope.props.a + scope.props.b).to_string());
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct SerializableScope {
    pub a: u64, // depends on user specified type
    pub b: u64, // depends on user specified type
    #[with(with::Skip)]
    pub text_nodes: [web_sys::Text; 3],
}
