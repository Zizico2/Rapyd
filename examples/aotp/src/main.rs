// Could try and rewrite using the following features. `const fn render` could then be part of a `Render` trait
// instead of just being implicitly required by the component macro
// #![feature(return_position_impl_trait_in_trait)]
// #![feature(const_trait_impl)]

// can use the following features to remove the need to enable lifecycle hooks
// #![feature(min_specialization)]
// #![feature(rustc_attrs)]

// include!("main.rs");
use std::{cell::RefCell, collections::HashSet, fmt::Display, marker::PhantomData};

#[rapyd_macros::test_use_attr]
use rapyd_macros::derived;
use syn::Ident;

fn main() {
    /*
    println!("{}", ComponentWithRender::__TEMPLATE.get(0).unwrap()());
    println!("{}", ComponentWithRender::__TEMPLATE.get(1).unwrap()());
    */
}

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
pub trait Template {
    fn get(&self, i: usize) -> Option<&fn() -> &'static str>;
}

impl Template for () {
    fn get(&self, i: usize) -> Option<&fn() -> &'static str> {
        None
    }
}

impl<const T: usize> Template for [fn() -> &'static str; T] {
    fn get(&self, i: usize) -> Option<&fn() -> &'static str> {
        <[fn() -> &'static str]>::get(self, i)
    }
}

// to enable lifecycle hooks access: #[rapyd_macros::component_test(lifecycle)]
// GO WITH THE SIMPLES VERSION. just prop(value) or state(value) is gooe enough for now.
// think of possible alternatives later or stick to this option
#[rapyd_macros::component_test]
pub struct Counter {
    #[prop(1)]
    //#[prop(or_else(|| -> 1))]
    step: u32,
    #[prop(0)]
    initial_count: u32,
    #[state(0)]
    count: RefCell<u32>,
    // prop/state
    // #[prop(or_default)]
    // #[prop(or(0))]
    // #[prop(or_else(|| { 0 }))]
}

impl Counter {
    fn increment_count(&self) {
        *self.count.borrow_mut() += 1;
    }
    const fn render() -> impl Template {
        let multiplied = derived!(|step: u32| *self.count.borrow() * step);

        let mut n = 0;

        while n < multiplied.dependencies.len() {
            let dep = multiplied.dependencies[n];

            n += 1;
        }

        /*
        effect!(|| log!(self.count.into()));

        html!(
            state_effect_
            <button @click={ self.increment_count }>
                "I count "{ multiplied!(5) }" !";
            </button>
        )
        let mut tok = TokenStream::new();
        for dep in multiplied.dependencies {

        }
        */
    }
}

struct Testss<const T: usize> {
    hi: u32,
}

const fn hi() -> usize {
    4
}
fn te() {
    let a = Testss::<{ hi() }> { hi: 7 };
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
    dependencies: [&'static str; AmountOfDependencies],
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
    test: [fn() -> ()]
}

impl TemplateEx for T {
    fn update_prop(&self, prop: usize) -> &dyn FnMut() -> () {
        &self.test[0]
    }
}