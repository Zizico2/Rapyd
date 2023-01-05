use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::component;

//TODO: this could be improved with array const generics -- [usize]
pub struct StateBase<
    const I: usize,
    const N_TEXT_NODES: usize,
    Scope: component::Scope<N_TEXT_NODES>,
> {
    _marker: PhantomData<Scope>,
}

pub trait State<const N_TEXT_NODES: usize, Scope: component::Scope<N_TEXT_NODES>> {
    fn on_update(sc: &Scope);
}

pub struct StateRefCell<
    const N_TEXT_NODES: usize,
    StateId: State<N_TEXT_NODES, Scope>,
    Scope: component::Scope<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    value: RefCell<Inner>,
    sc: Rc<Scope>,
}

impl<
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > StateRefCell<N_TEXT_NODES, StateId, Scope, Inner>
{
    /// initializes the struct. doesn't access the values in any way whatsoever
    pub fn new(inner: Inner, sc: Rc<Scope>) -> Self {
        Self {
            __marker: PhantomData,
            value: RefCell::new(inner),
            sc,
        }
    }
}

pub struct StateRef<
    'a,
    const N_TEXT_NODES: usize,
    StateId: State<N_TEXT_NODES, Scope>,
    Scope: component::Scope<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_: Ref<'a, Inner>,
    sc: Rc<Scope>,
}
pub struct StateRefMut<
    'a,
    const N_TEXT_NODES: usize,
    StateId: State<N_TEXT_NODES, Scope>,
    Scope: component::Scope<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_mut: RefMut<'a, Inner>,
    sc: Rc<Scope>,
}

impl<
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > StateRefCell<N_TEXT_NODES, StateId, Scope, Inner>
{
    pub fn borrow(&self) -> StateRef<'_, N_TEXT_NODES, StateId, Scope, Inner> {
        StateRef {
            __marker: PhantomData,
            ref_: self.value.borrow(),
            sc: self.sc.clone(),
        }
    }
    pub fn borrow_mut(&self) -> StateRefMut<'_, N_TEXT_NODES, StateId, Scope, Inner> {
        StateRefMut {
            __marker: PhantomData,
            ref_mut: self.value.borrow_mut(),
            sc: self.sc.clone(),
        }
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > Deref for StateRef<'a, N_TEXT_NODES, StateId, Scope, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > Deref for StateRefMut<'a, N_TEXT_NODES, StateId, Scope, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_mut.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > DerefMut for StateRefMut<'a, N_TEXT_NODES, StateId, Scope, Inner>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_mut.deref_mut()
    }
}

impl<
        const N_TEXT_NODES: usize,
        StateId: State<N_TEXT_NODES, Scope>,
        Scope: component::Scope<N_TEXT_NODES>,
        Inner,
    > Drop for StateRefMut<'_, N_TEXT_NODES, StateId, Scope, Inner>
{
    fn drop(&mut self) {
        StateId::on_update(&self.sc);
    }
}
