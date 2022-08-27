use std::{
    array,
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
    rc::Rc,
    slice,
};

pub trait UpdateState<S: StateTag<T>, T> {
    fn update_state(&self, new: &T);
}

pub trait StateTag<T>: Into<T> {}

#[derive(Debug)]
pub struct ScopeState<T, S: StateTag<T>, U: UpdateState<S, T>> {
    pub _marker: PhantomData<S>,
    pub updater: Rc<U>,
    pub val: Rc<RefCell<T>>,
}


impl<T, S: StateTag<T>, U: UpdateState<S, T>> Clone for ScopeState<T, S, U> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker.clone(),
            updater: self.updater.clone(),
            val: self.val.clone(),
        }
    }
}

impl<T, S: StateTag<T>, U: UpdateState<S, T>> ScopeState<T, S, U> {
    pub fn new(val: S, updater: Rc<U>) -> Self {
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

impl<T, S: StateTag<T>, U: UpdateState<S, T>> ScopeState<T, S, U> {
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
    updater: Rc<U>,
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
    fn new(state: &'a ScopeState<T, S, U>) -> Self {
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

// TODO
// LOOK AT THE PERFORMANCE OF THIS
pub fn split_array<const LEFT: usize, const RIGHT: usize, const LEN: usize, T>(
    arr: [T; LEN],
) -> ([T; LEFT], [T; RIGHT]) {
    assert_eq!(LEFT + RIGHT, LEN);
    let mut left: [MaybeUninit<T>; LEFT] = array::from_fn(|_| MaybeUninit::uninit());
    let mut right: [MaybeUninit<T>; RIGHT] = array::from_fn(|_| MaybeUninit::uninit());

    for (i, val) in arr.into_iter().enumerate() {
        if i < LEFT {
            left[i] = MaybeUninit::new(val);
        } else {
            right[i - LEFT] = MaybeUninit::new(val);
        }
    }

    (
        left.map(|val| unsafe { val.assume_init() }),
        right.map(|val| unsafe { val.assume_init() }),
    )
}
