// Could try and rewrite using the following features. `const fn render` could then be part of a `Render` trait
// instead of just being implicitly required by the component macro
// #![feature(return_position_impl_trait_in_trait)]
// #![feature(const_trait_impl)]

// can use the following features to remove the need to enable lifecycle hooks
// #![feature(min_specialization)]
// #![feature(rustc_attrs)]

// include!("main.rs");
use std::{fmt::Display, marker::PhantomData};

use rapyd_macros::derived;

fn main() {
    /*
    println!("{}", ComponentWithRender::__TEMPLATE.get(0).unwrap()());
    println!("{}", ComponentWithRender::__TEMPLATE.get(1).unwrap()());
    */
}

trait Lifecycle {
    fn on_mount(scope: &Self) {}
    fn on_cleanup(scope: &Self) {}
}
pub trait Template {
    fn get(&self, i: usize) -> Option<&fn() -> &'static str>;
}

impl<const T: usize> Template for [fn() -> &'static str; T] {
    fn get(&self, i: usize) -> Option<&fn() -> &'static str> {
        <[fn() -> &'static str]>::get(self, i)
    }
}

// to enable lifecycle hooks access: #[rapyd_macros::component_test(lifecycle)]
#[derive(Default)]
#[rapyd_macros::component_test]
pub struct Counter {
    #[prop(or(1))]
    //#[prop(or_else(|| -> 1))]
    step: u32,
    #[prop(default)]
    initial_count: u32,
    #[state]
    count: u32,
}

impl Counter {
    fn init_count(&self) -> u32 {
        self.initial_count
    }
    fn increment_count(&self) {
        *self.count.borrow_mut() += 1;
    }
    const fn render() -> impl Template {
        let multiplied = derived!(|cx, step: u32| cx.count * step);

        effect!(|cx| log!(cx.count.into()));

        html!(
            <button @click={ cx.increment_count }>
                "I count "{ multiplied(cx, 5) }" !";
            </button>
        )
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

struct Derived<T>(T);

impl<T> Data for Derived<T> {}
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
}
