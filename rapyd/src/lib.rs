use std::{
    cell::{RefCell, RefMut},
    fmt::Display,
    ops::{Deref, DerefMut},
};
pub use rapyd_macros;

pub struct StateCell<'a, ValueType, ScopeType: Scope, StateTagType: Copy + StateTag<ScopeType>> {
    value: RefCell<ValueType>,
    state_tag: StateTagType,
    scope: &'a ScopeType,
}

impl<'a, ValueType, ScopeType: Scope, StateTagType: Copy + StateTag<ScopeType>>
    StateCell<'a, ValueType, ScopeType, StateTagType>
{
    fn borrow_mut(&self) -> StateMut<'a, '_, ValueType, ScopeType, StateTagType> {
        StateMut {
            value: self.value.borrow_mut(),
            inner_state: self.state_tag,
            scope: self.scope,
        }
    }
}

impl<'a, ValueType: ToDomString, ScopeType: Scope, StateTagType: Copy + StateTag<ScopeType>>
    ToDomString for StateCell<'a, ValueType, ScopeType, StateTagType>
{
    fn to_dom_string(&self) -> String {
        self.value.borrow().to_dom_string()
    }
}

pub struct State<'a, ValueType> {
    value: RefMut<'a, ValueType>,
}

impl<ValueType> Deref for State<'_, ValueType> {
    type Target = ValueType;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<ValueType: ToDomString> ToDomString for State<'_, ValueType> {
    fn to_dom_string(&self) -> String {
        self.value.to_dom_string()
    }
}

pub struct StateMut<'a, 'b, ValueType, ScopeType: Scope, StateTagType: StateTag<ScopeType>> {
    value: RefMut<'b, ValueType>,
    inner_state: StateTagType,
    scope: &'a ScopeType,
}

impl<ValueType, ScopeType: Scope, StateTagType: StateTag<ScopeType>> Drop
    for StateMut<'_, '_, ValueType, ScopeType, StateTagType>
{
    fn drop(&mut self) {
        self.inner_state.on_change(self.scope);
    }
}

impl<ValueType, ScopeType: Scope, StateTagType: StateTag<ScopeType>> Deref
    for StateMut<'_, '_, ValueType, ScopeType, StateTagType>
{
    type Target = ValueType;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<ValueType, ScopeType: Scope, StateTagType: StateTag<ScopeType>> DerefMut
    for StateMut<'_, '_, ValueType, ScopeType, StateTagType>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.deref_mut()
    }
}

impl<ValueType: ToDomString, ScopeType: Scope, StateTagType: StateTag<ScopeType>> ToDomString
    for StateMut<'_, '_, ValueType, ScopeType, StateTagType>
{
    fn to_dom_string(&self) -> String {
        self.value.to_dom_string()
    }
}

pub trait StateTag<ScopeType: Scope> {
    fn on_change(&self, scope: &ScopeType);
}

pub trait Scope {}

pub trait ToDomString {
    fn to_dom_string(&self) -> String;
}

impl<T: Display> ToDomString for T {
    fn to_dom_string(&self) -> String {
        self.to_string()
    }
}
/*
#[component]
mod Counter {
    // can access the vars diretly or use "&self"

    #[scope]
    struct Test {
        // #[prop]
        #[state]
        count: i32,
    }

    /*
    html! {
        <button on:click={ increment_count }>
            "You clicked me this many times: " { count }
        </button>
        /*
        <button on:click={ increment_count }>
            "You clicked me this many times: " { count }
        </button>
        */
    }

    impl Scope {
        // can access the vars directly or use "&self" or "scope: scope!()"
        fn increment_count(&self) {
            //let mut count = self.count.borrow_mut();
            // *count += 1;
        }
    }

    //#[on_change(count)]
    fn log_changes() {
        //println!(count);
    }
    */
}
/*
#[cfg(test)]
mod test {
    use rapyd_macros::component;

    #[test]
    fn test1() {
        const RIGHT_HTML: &str = "<div><div></div><div><div></div></div></div><div></div><div><div><div></div><div></div></div><div></div><div><div></div></div><div></div></div>";
        #[component]
        mod Test {
            html! {<div><div></div><div><div></div></div></div><div></div><div><div><div></div><div></div></div><div></div><div><div></div></div><div></div></div>}
        }
        assert_eq!(RIGHT_HTML, Test::__html_template::TEMPLATE);
    }
    #[test]
    fn test2() {
        const RIGHT_HTML: &str = "<span><!><span></span><div><div><!></div></div>hi</span><div></div><div><div>hi <div></div><div><!></div></div><span></span><div><div></div><!></div><div></div></div>";
        #[component]
        mod Test {
            html! {<span>{""}<span></span><div><div>{""}</div></div>"hi"</span><div></div><div><div>"hi "<div></div><div>{""}</div></div><span></span><div><div></div>{""}</div><div></div></div>}
        }
        assert_eq!(RIGHT_HTML, Test::__html_template::TEMPLATE);
    }
}
*/
*/
