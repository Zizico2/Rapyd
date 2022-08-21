use std::ops::{Deref, DerefMut};
/*
pub trait State<'a, T, M: StateMut<T, M>>: Deref<Target = T> {
    fn borrow_mut(&'a mut self) -> M;
}

// Drop bound here so I don't forget to impl Drop for State types
#[allow(drop_bounds)]
pub trait StateMut<T, M: StateMut<T, M>>: Deref<Target = T> + DerefMut + Drop {}
*/