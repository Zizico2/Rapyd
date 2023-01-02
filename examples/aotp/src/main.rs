// Could try and rewrite using the following features. `const fn render` could then be part of a `Render` trait
// instead of just being implicitly required by the component macro
// #![feature(return_position_impl_trait_in_trait)]
// #![feature(const_trait_impl)]

// can use the following features to remove the need to enable lifecycle hooks
// #![feature(min_specialization)]
// #![feature(rustc_attrs)]
#![feature(const_mut_refs)]
#![feature(const_replace)]
#![feature(const_maybe_uninit_write)]
// #![feature(const_option)]
// #![feature(const_option_ext)]
//#![feature(stmt_expr_attributes)]

use std::cell::{Ref, RefMut};
use std::mem::{self, MaybeUninit};
use std::rc::Rc;
// include!("main.rs");
use std::{cell::RefCell, collections::HashSet, fmt::Display, marker::PhantomData};

mod state;

#[rapyd_macros::test_use_attr]
use rapyd_macros::derived;
use state::StateRefCell;
use syn::Ident;

fn main() {
    /*
    println!("{}", ComponentWithRender::__TEMPLATE.get(0).unwrap()());
    println!("{}", ComponentWithRender::__TEMPLATE.get(1).unwrap()());
    */
}

const FUNC: &dyn Fn() = &|| {};
const FUNC_2: fn() -> () = || {};
fn func_3() {}

trait Lifecycle
where
    Self: Sized,
{
    fn on_new(self) -> Self {
        self
    }
    fn on_mount(scope: &Self) {}
    fn on_cleanup(scope: &Self) {}
}
pub trait Template<S: Component> {
    fn get_updater(&self, i: usize) -> fn(self_: Rc<S>) -> &'static str;
}

impl<S: Component> Template<S> for () {
    fn get_updater(&self, i: usize) -> fn(self_: Rc<S>) -> &'static str {
        todo!()
    }
}

impl<const T: usize, S: Component> Template<S> for [fn() -> &'static str; T] {
    fn get_updater(&self, i: usize) -> fn(self_: Rc<S>) -> &'static str {
        todo!()
    }
}

// to enable lifecycle hooks access: #[rapyd_macros::component_test(lifecycle)]
// GO WITH THE SIMPLES VERSION. just prop(value) or state(value) is gooe enough for now.
// think of possible alternatives later or stick to this option

//? #[rapyd_macros::component_test]
pub struct Counter {
    // #[state]
    count: StateRefCell<0, Self, u32>,
}

impl Component for Counter {
    const TEMPLATE: &'static dyn Template<Counter> = todo!();
}

pub trait Component: /* Sized  + */ 'static {
    const TEMPLATE: &'static dyn Template<Self>;
}
/*
struct StateRefCell<const STATE_INDEX: usize, S: Component, Inner> {
    value: RefCell<Inner>,
    scope: Rc<S>,
}

impl<const STATE_INDEX: usize, S: Component, Inner> StateRefCell<STATE_INDEX, S, Inner> {
    fn borrow(&self) -> StateRef<'_, STATE_INDEX, S, Inner> {
        StateRef {
            ref_: self.value.borrow(),
            scope: self.scope.clone(),
        }
    }
    fn borrow_mut(&self) -> StateRefMut<'_, STATE_INDEX, S, Inner> {
        StateRefMut {
            ref_mut: self.value.borrow_mut(),
            scope: self.scope.clone(),
        }
    }
}

struct StateRefMut<'a, const STATE_INDEX: usize, S: Component, Inner> {
    ref_mut: RefMut<'a, Inner>,
    scope: Rc<S>,
}

struct StateRef<'a, const STATE_INDEX: usize, S: Component, Inner> {
    ref_: Ref<'a, Inner>,
    scope: Rc<S>,
}

impl<const STATE_INDEX: usize, S: Component, Inner> Drop for StateRefMut<'_, STATE_INDEX, S, Inner> {
    fn drop(&mut self) {
        S::TEMPLATE.get_updater(STATE_INDEX)(self.scope.clone());
    }
}
 */
impl Counter {
    const __STEP: usize = 0;
    const __INITIAL_COUNT: usize = 1;
    const __COUNT: usize = 2;

    // const __STATE_EFFECTS: [&dyn Fn(&Self) -> (); 3] = [&|_| {}, &|_| {}, &|_| {}];
    const __STATE_FIELDS: usize = 3;

    const fn field_name_to_index(field_name: &'static str) -> usize {
        if const_str::equal!(field_name, "step") {
            Self::__STEP
        } else if const_str::equal!(field_name, "initial_count") {
            Self::__INITIAL_COUNT
        } else if const_str::equal!(field_name, "count") {
            Self::__COUNT
        } else {
            panic!("field not existant")
        }
    }
    //const __TEST: [[(); 1]; 1] = [[()]];
}

pub struct TextNode {}

impl TextNode {
    fn update(&self) {}
}

impl Counter {
    fn increment_count(&self) {
        // *self.count.borrow_mut() += 1;
    }
    const fn render() -> impl Template<Self> {
        // let a = #[inline(always)] || {};
        let multiplied = derived!(|step: u32| *self.count.borrow() * 0);
        let __multipled_dependency_mapping: [bool; Self::__STATE_FIELDS] = [false, false, true];

        const __N_TEXT_NODES: usize = 1;

        let mut on_state_change: [[bool; __N_TEXT_NODES]; Self::__STATE_FIELDS] =
            [[false], [false], [false]];
        /*
        effect!(|| log!(self.count.into()));

        html!(
            <button @click={ self.increment_count }>
                "I count "{ multiplied!(5) }" !";
            </button>
        )

        let mut tok = TokenStream::new();
        for dep in multiplied.dependencies {

        }
        */
        on_state_change[0][0] = __multipled_dependency_mapping[0] | on_state_change[0][0];
        on_state_change[1][0] = __multipled_dependency_mapping[1] | on_state_change[1][0];
        on_state_change[2][0] = __multipled_dependency_mapping[2] | on_state_change[2][0];

        let on_update = |cx: &Self, updated_state: usize| {
            let mut n = 0;
            while n < on_state_change[updated_state].len() {
                if on_state_change[updated_state][n] {
                    // cx.text_nodes[n].update();
                }
                n += 1;
            }
        };

        on_state_change
    }
}

impl<const T: usize, const U: usize, S: Component> Template<S> for [[bool; T]; U] {
    fn get_updater(&self, i: usize) -> fn(self_: Rc<S>) -> &'static str {
        todo!()
    }
}

/*
#[rapyd_macros::component_test]
pub struct Counter {
    #[prop]
    #[state]
    count: u32,
}

impl Counter {
    fn increment_count(&mut self) {
        self.count += 1;
    }
    const fn render() -> impl Template {
        let multiplied = derived!(|cx, step: i32| cx.count * step);

        effect!(|cx| log!(cx.count.into()));

        html!(
            <button @click={ cx.increment_count }>
                "I count "{ multiplied(cx, 5) }" !";
            </button>
        )
    }
}
*/

struct __Derived<const AmountOfDependencies: usize, T> {
    closure: T,
    dependencies: [usize; AmountOfDependencies],
}

impl<const AmountOfDependencies: usize, T> Data for __Derived<AmountOfDependencies, T> {}
impl<T: Display> Data for T {}

trait Data {}

struct Test {}

struct Props;
impl Test {
    fn test() {
        let a = Some(());
        let a = |a| {
            let a: Self = a;
        };
    }

    fn h1(&self) {
        Test::h2(self)
    }

    fn h2(&self) {}
}

use syn::__private::Span;

trait TemplateEx {
    fn update_prop(&self, prop: usize) -> &dyn FnMut() -> ();
}

struct T {
    test: [fn() -> ()],
}

impl TemplateEx for T {
    fn update_prop(&self, prop: usize) -> &dyn FnMut() -> () {
        &self.test[0]
    }
}
