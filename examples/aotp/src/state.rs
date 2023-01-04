use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::component;

//TODO: this could be improved with array const generics -- [usize]
pub struct State<
    const I: usize,
    const N_TEXT_NODES: usize,
    Context: component::Context<N_TEXT_NODES>,
> {
    _marker: PhantomData<Context>,
}

pub trait OnUpdate<const N_TEXT_NODES: usize, Context: component::Context<N_TEXT_NODES>> {
    fn on_update(context: Rc<Context>);
}

pub struct StateRefCell<
    const N_TEXT_NODES: usize,
    StateId: OnUpdate<N_TEXT_NODES, Context>,
    Context: component::Context<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    value: RefCell<Inner>,
    scope: Rc<Context>,
}

pub struct StateRef<
    'a,
    const N_TEXT_NODES: usize,
    StateId: OnUpdate<N_TEXT_NODES, Context>,
    Context: component::Context<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_: Ref<'a, Inner>,
    scope: Rc<Context>,
}
pub struct StateRefMut<
    'a,
    const N_TEXT_NODES: usize,
    StateId: OnUpdate<N_TEXT_NODES, Context>,
    Context: component::Context<N_TEXT_NODES>,
    Inner,
> {
    __marker: PhantomData<StateId>,
    ref_mut: RefMut<'a, Inner>,
    scope: Rc<Context>,
}

impl<
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > StateRefCell<N_TEXT_NODES, StateId, Context, Inner>
{
    pub fn new(value: Inner, scope: Rc<Context>) -> Self {
        Self {
            __marker: PhantomData,
            value: RefCell::new(value),
            scope,
        }
    }
}

impl<
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > StateRefCell<N_TEXT_NODES, StateId, Context, Inner>
{
    pub fn borrow(&self) -> StateRef<'_, N_TEXT_NODES, StateId, Context, Inner> {
        StateRef {
            __marker: PhantomData,
            ref_: self.value.borrow(),
            scope: self.scope.clone(),
        }
    }
    pub fn borrow_mut(&self) -> StateRefMut<'_, N_TEXT_NODES, StateId, Context, Inner> {
        StateRefMut {
            __marker: PhantomData,
            ref_mut: self.value.borrow_mut(),
            scope: self.scope.clone(),
        }
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > Deref for StateRef<'a, N_TEXT_NODES, StateId, Context, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > Deref for StateRefMut<'a, N_TEXT_NODES, StateId, Context, Inner>
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.ref_mut.deref()
    }
}

impl<
        'a,
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > DerefMut for StateRefMut<'a, N_TEXT_NODES, StateId, Context, Inner>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_mut.deref_mut()
    }
}

impl<
        const N_TEXT_NODES: usize,
        StateId: OnUpdate<N_TEXT_NODES, Context>,
        Context: component::Context<N_TEXT_NODES>,
        Inner,
    > Drop for StateRefMut<'_, N_TEXT_NODES, StateId, Context, Inner>
{
    fn drop(&mut self) {
        StateId::on_update(self.scope.clone());
    }
}
