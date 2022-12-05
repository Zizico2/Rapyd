// Could try and rewrite using the following features. `const fn render` could then be part of a `Render` trait
// instead of just being implicitly required by the component macro
// #![feature(return_position_impl_trait_in_trait)]
// #![feature(const_trait_impl)]

// can use the following features to remove the need to enable lifecycle hooks
// #![feature(min_specialization)]
// #![feature(rustc_attrs)]

use std::{fmt::Display, marker::PhantomData};

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
#[rapyd_macros::component_test]
pub struct Counter {
    //#[prop]
    //#[state]
    count: u32,
}

impl Counter {
    const fn render() -> impl Template {
        let a = |a| {
            let b: u32 = a;
        };
        let a = || "a";
        let b = || "2";
        let c: [fn() -> &'static str; 2] = [a, b];
        c
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
    const fn render() -> impl Template {
        let multiplied = derived!(|cx: Self, step: i32| cx.count * step);
        html! {
            <button>
                "I count "{ multiplied(cx, 5) }" !";
            </button>
        }
    }
}
*/

struct Derived<T> {
    _marker: PhantomData<T>,
}

impl<T> Data for Derived<T> {}
impl<T: Display> Data for T {}

trait Data {}
