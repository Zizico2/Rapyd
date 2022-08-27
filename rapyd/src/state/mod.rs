use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub mod closure;

pub trait UpdateState<S: StateTag<T>, T>: Clone {
    fn update_state(&self, new: &T);
}

pub trait StateTag<T>: Into<T> {}

#[derive(Debug)]
pub struct State<T, S: StateTag<T>, U: UpdateState<S, T>> {
    pub _marker: PhantomData<S>,
    pub updater: U,
    pub val: Rc<RefCell<T>>,
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> Clone for State<T, S, U> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker.clone(),
            updater: self.updater.clone(),
            val: self.val.clone(),
        }
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> State<T, S, U> {
    pub fn new(val: S, updater: U) -> Self {
        Self {
            _marker: PhantomData,
            updater,
            val: Rc::new(RefCell::new(val.into())),
        }
    }

    pub fn update_state(&self) {
        self.updater.update_state(&*self.val.deref().borrow());
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> State<T, S, U> {
    pub fn borrow_mut(&self) -> StateMut<T, S, U> {
        StateMut::new(self)
    }

    pub fn borrow(&self) -> Ref<T> {
        self.val.deref().borrow()
    }
}

pub struct StateMut<'a, T, S: StateTag<T>, U: UpdateState<S, T>> {
    _marker: PhantomData<S>,
    val: RefMut<'a, T>,
    updater: U,
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> Deref for StateMut<'_, T, S, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> DerefMut for StateMut<'_, T, S, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.val
    }
}

impl<'a, T, S: StateTag<T>, U: UpdateState<S, T>> StateMut<'a, T, S, U> {
    fn new(state: &'a State<T, S, U>) -> Self {
        Self {
            _marker: PhantomData,
            val: state.val.deref().borrow_mut(),
            updater: state.updater.clone(),
        }
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> Drop for StateMut<'_, T, S, U> {
    fn drop(&mut self) {
        self.updater.update_state(&*self.val);
    }
}
