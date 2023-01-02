use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::Component;

pub struct StateRefCell<const STATE_INDEX: usize, S: Component, Inner> {
    value: RefCell<Inner>,
    scope: Rc<S>,
}

impl<const STATE_INDEX: usize, S: Component, Inner> StateRefCell<STATE_INDEX, S, Inner> {
    pub fn borrow(&self) -> StateRef<'_, STATE_INDEX, S, Inner> {
        StateRef {
            ref_: self.value.borrow(),
            scope: self.scope.clone(),
        }
    }
    pub fn borrow_mut(&self) -> StateRefMut<'_, STATE_INDEX, S, Inner> {
        StateRefMut {
            ref_mut: self.value.borrow_mut(),
            scope: self.scope.clone(),
        }
    }
}

pub struct StateRefMut<'a, const STATE_INDEX: usize, S: Component, Inner> {
    ref_mut: RefMut<'a, Inner>,
    scope: Rc<S>,
}

pub struct StateRef<'a, const STATE_INDEX: usize, S: Component, Inner> {
    ref_: Ref<'a, Inner>,
    scope: Rc<S>,
}

impl<const STATE_INDEX: usize, S: Component, Inner> Drop
    for StateRefMut<'_, STATE_INDEX, S, Inner>
{
    fn drop(&mut self) {
        S::TEMPLATE.get_updater(STATE_INDEX)(self.scope.clone());
    }
}

impl<'a, const STATE_INDEX: usize, S: Component, Inner> Deref
    for StateRef<'a, STATE_INDEX, S, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

impl<'a, const STATE_INDEX: usize, S: Component, Inner> Deref
    for StateRefMut<'a, STATE_INDEX, S, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_mut.deref()
    }
}

impl<'a, const STATE_INDEX: usize, S: Component, Inner> DerefMut
    for StateRefMut<'a, STATE_INDEX, S, Inner>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_mut.deref_mut()
    }
}
