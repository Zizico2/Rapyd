use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub trait UpdateState<S: StateTag<T>, T> {
    fn update_state(&self, new: &T);
}

pub trait StateTag<T>: Into<T> {}

pub struct ScopeState<T, S: StateTag<T>, U: UpdateState<S, T>> {
    pub _marker: PhantomData<S>,
    pub updater: Rc<U>,
    pub val: T,
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> ScopeState<T, S, U> {
    /*
        may only call the destructor of the value returned by borrow_mut after setting updater
    */
    pub fn new(val: S, updater: Rc<U>) -> Self {
        Self {
            _marker: PhantomData,
            updater,
            val: val.into(),
        }
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> ScopeState<T, S, U> {
    fn on_mut_drop(&self, new: &T) {
        self.updater.update_state(new);
    }
    pub fn borrow_mut(&mut self) -> StateMut<T, S, U> {
        StateMut::new(self)
    }

    pub fn borrow(&self) -> &T {
        &self.val
    }
}

pub struct StateMut<'a, T, S: StateTag<T>, U: UpdateState<S, T>> {
    state: &'a mut ScopeState<T, S, U>,
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> Deref for StateMut<'_, T, S, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state.val
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> DerefMut for StateMut<'_, T, S, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state.val
    }
}

impl<'a, T, S: StateTag<T>, U: UpdateState<S, T>> StateMut<'a, T, S, U> {
    fn new(state: &'a mut ScopeState<T, S, U>) -> Self {
        Self { state }
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> Drop for StateMut<'_, T, S, U> {
    fn drop(&mut self) {
        self.state.on_mut_drop(&self.state.val);
    }
}
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/*
struct State0(u32);

impl StateTag<u32> for State0 {}

impl Deref for State0 {
    type Target = RefCell<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UpdateState<State0, u32> for Scope {
    fn update_state(&self, new: &RefMut<u32>) {
        dbg!(new);
    }
}

struct Scope;
/*
fn main() {
    let state = {
        let val = State0(RefCell::new(20));
        let updater = Rc::new(Scope);
        ScopeState {
            _marker: PhantomData,
            updater: updater.clone(),
            val,
        }
    };
    {
        let mut aux = state.borrow_mut();
        *aux = 50;
    }

    {
        let mut aux = state.borrow_mut();
        *aux += 20;
    }
}
 */
*/
