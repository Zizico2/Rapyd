use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct State<T> {
    pub val: Rc<RefCell<T>>,
    pub reactions: Vec<Rc<RefCell<dyn FnMut(&T)>>>,
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            reactions: self.reactions.clone(),
        }
    }
}

impl<T> State<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: Rc::new(RefCell::new(val.into())),
            reactions: vec![],
        }
    }

    pub fn update_state(&mut self) {
        for func in &self.reactions {
            let mut func = func.borrow_mut();
            func(&*self.val.deref().borrow());
        }
    }
    pub fn push_reaction(&mut self, reaction: Rc<RefCell<dyn FnMut(&T)>>) {
        self.reactions.push(reaction);
    }
}

impl<T> State<T> {
    pub fn borrow_mut(&mut self) -> StateMut<T> {
        StateMut::new(self)
    }

    pub fn borrow(&self) -> Ref<T> {
        self.val.deref().borrow()
    }
}

pub struct StateMut<'a, T> {
    reactions: Vec<Rc<RefCell<dyn FnMut(&T)>>>,
    val: RefMut<'a, T>,
}

impl<T> Deref for StateMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

impl<T> DerefMut for StateMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.val
    }
}

impl<'a, T> StateMut<'a, T> {
    fn new(state: &'a mut State<T>) -> Self {
        Self {
            val: state.val.deref().borrow_mut(),
            reactions: state.reactions.clone(),
        }
    }
}

impl<T> Drop for StateMut<'_, T> {
    fn drop(&mut self) {
        for func in self.reactions.deref() {
            let mut func = func.borrow_mut();
            func(&*self.val.deref());
        }
    }
}
