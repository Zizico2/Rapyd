#![feature(new_uninit)]
#![feature(generic_arg_infer)]

mod component;
mod state;
//#[component]
mod number_display {
    use crate::component::Scope as _;

    use crate::component::ScopeBase;
    use crate::component::TextNode;
    use crate::state::StateBase;

    use std::mem::transmute;
    use std::mem::MaybeUninit;
    use std::ptr::addr_of_mut;
    use std::rc::Rc;

    use crate::component;
    use crate::state;
    use crate::state::StateRefCell;

    struct State {
        count: u32,
    }
    impl component::Context for Context {}
    impl component::Scope<1> for Scope {
        fn get_text_nodes(&self) -> &[web_sys::Text; Self::N_TEXT_NODES] {
            &self.text_nodes
        }

        type Context = Context;

        fn get_context(&self) -> &Self::Context {
            &self.cx
        }
    }

    // REPEAT FOR EVERY STATE VAR
    impl state::State<{ Scope::N_TEXT_NODES }, Scope>
        for state::StateBase<0, { Scope::N_TEXT_NODES }, Scope>
    {
        fn on_update(cx: &Scope) {
            // REPEAT THIS LINE FOR EVERY TEXT NODE THAT DEPPENDS ON THIS STATE VAR
            cx.update_text_node::<0>();
            // ADD MORE PRESET EFFECTS (lifecycle hooks, maybe more)
        }
    }

    type TextNodeBase<const I: usize> = component::TextNodeBase<I, { Scope::N_TEXT_NODES }, Scope>;

    // REPEAT FOR EVERY TEXT NODE
    impl component::TextNode<{ Scope::N_TEXT_NODES }, Scope>
        for component::TextNodeBase<0, { Scope::N_TEXT_NODES }, Scope>
    {
        fn get_data(cx: &Context) -> String {
            let val = { cx.multiplied(3) };
            component::ToData::to_data(&val)
        }
    }

    impl Scope {
        // TODO implementation without nightly (#![feature(new_uninit)]) should be possible
        pub fn new(initial_count: u32, step: Option<u32>) -> Rc<Self> {
            let step: u32 = Option::unwrap_or_default(step);
            let State { count } = init_state(&initial_count, &step);
            let mut domless_context: MaybeUninit<Context> = MaybeUninit::uninit();
            let domless_context_ptr: *mut Context = domless_context.as_mut_ptr();
            unsafe {
                addr_of_mut!((*domless_context_ptr).initial_count).write(initial_count);
            }
            unsafe {
                addr_of_mut!((*domless_context_ptr).step).write(step);
            }
            let mut context: Rc<MaybeUninit<Self>> = Rc::new_uninit();

            let context_ptr: *mut Scope = Rc::get_mut(&mut context).unwrap().as_mut_ptr();

            let count: Count = {
                let context = context.clone();
                Count::new(count, unsafe { transmute(context) })
            };

            unsafe {
                addr_of_mut!((*domless_context_ptr).count).write(count);
            }

            let domless_context = unsafe { domless_context.assume_init() };

            type T<const A: usize> = component::TextNodeBase<A, { Scope::N_TEXT_NODES }, Scope>;

            let text_nodes: [web_sys::Text; Self::N_TEXT_NODES] =
                rapyd_macros::arr!(|I, 1| web_sys::Text::new_with_data(&T::<I>::get_data(
                    &domless_context
                ))
                .unwrap());

            unsafe {
                addr_of_mut!((*context_ptr).cx).write(domless_context);
            }

            unsafe {
                addr_of_mut!((*context_ptr).text_nodes).write(text_nodes);
            }

            unsafe { context.assume_init() }
        }
    }

    type Count = StateRefCell<
        { Scope::N_TEXT_NODES },
        StateBase<0, { Scope::N_TEXT_NODES }, Scope>,
        Scope,
        u32,
    >;

    pub struct Context {
        //#[prop]
        initial_count: u32,
        //#[prop(default)]
        step: u32,
        //#[state]
        count: Count,
    }

    type Scope = ScopeBase<1, Context>;

    fn init_state(initial_count: &u32, step: &u32) -> State {
        State {
            count: *initial_count,
        }
    }
    impl Context {
        fn multiplied(&self, factor: u32) -> u32 {
            *self.count.borrow() * factor
        }
    }

    /*
    html! {
        <div> "I count "{ cx.multiplied(3) }"!" </div>
    };
    */
}

fn main() {
    //let test: [usize; 4] = arr!(|I, 4| I);
}
