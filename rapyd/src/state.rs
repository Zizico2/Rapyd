use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::component::{self, ToData};

//TODO: this could be improved with array const generics -- [usize]
pub struct StateBase<
    const I: usize,
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
    Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
> {
    _marker: PhantomData<Scope>,
}

pub trait State<
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
    Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
>
{
    fn on_update(sc: &Scope);
}

pub struct StateRefCell<
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
    StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
    Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    value: RefCell<Inner>,
    sc: Rc<Scope>,
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > StateRefCell<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
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
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
    StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
    Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_: Ref<'a, Inner>,
    _sc: Rc<Scope>,
}
pub struct StateRefMut<
    'a,
    const N_TEXT_NODES: usize,
    const N_WALKS: usize,
    const N_EVENT_LISTENERS: usize,
    StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
    Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_mut: ManuallyDrop<RefMut<'a, Inner>>,
    sc: Rc<Scope>,
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > StateRefCell<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    pub fn borrow(
        &self,
    ) -> StateRef<'_, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner> {
        //? ERROR this is being called when the state is already mutably borrowed
        //self.value
        StateRef {
            __marker: PhantomData,
            ref_: self.value.borrow(),
            _sc: self.sc.clone(),
        }
    }
    pub fn borrow_mut(
        &self,
    ) -> StateRefMut<'_, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner> {
        StateRefMut {
            __marker: PhantomData,
            ref_mut: ManuallyDrop::new(self.value.borrow_mut()),
            sc: self.sc.clone(),
        }
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > Deref for StateRef<'a, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > Deref for StateRefMut<'a, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_mut.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > DerefMut
    for StateRefMut<'a, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_mut.deref_mut()
    }
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner,
    > Drop for StateRefMut<'_, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    fn drop(&mut self) {
        //drop(self.ref_mut);
        unsafe { ManuallyDrop::drop(&mut self.ref_mut) }

        StateId::on_update(&self.sc);
    }
}

impl<
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner: ToData,
    > ToData for StateRefCell<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    fn to_data(&self) -> String {
        self.value.borrow().to_data()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner: ToData,
    > ToData for StateRef<'a, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    fn to_data(&self) -> String {
        self.ref_.borrow().to_data()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        const N_WALKS: usize,
        const N_EVENT_LISTENERS: usize,
        StateId: State<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, Scope>,
        Scope: component::Scope<N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS>,
        Inner: ToData,
    > ToData for StateRefMut<'a, N_TEXT_NODES, N_WALKS, N_EVENT_LISTENERS, StateId, Scope, Inner>
{
    fn to_data(&self) -> String {
        self.ref_mut.borrow().to_data()
    }
}
