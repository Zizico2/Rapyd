#![feature(new_uninit)]

mod component;
mod state;
//#[component]
mod number_display {
    /*
        use crate::component::Scope as _;

        use crate::component::ScopeBase;

        use crate::component::WithProps;
        use crate::component::WithTextNode;

        use std::mem::transmute;
        use std::mem::MaybeUninit;
        use std::ptr::addr_of_mut;
        use std::rc::Rc;

        use crate::component;
        use crate::state;
    */
    // USER WRITTEN - START

    pub struct Context {
        //#[prop]
        initial_count: u32,
        //#[prop(default)]
        step: u32,
        //#[state]
        count: StateRefCell<0>,
    }

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

    // USER WRITTEN - END

    // MACRO GERATED - START

    // CONTEXT - START
    impl crate::component::Context<1> for Context {
        type Scope = Scope;
    }

    /// RAW STATE
    struct State {
        count: u32,
    }

    /// RAW PROPS
    pub struct Props {
        pub initial_count: u32,
        pub step: Option<u32>,
    }
    impl crate::component::Props for Props {
        type ProcessedProps = ProcessedProps;
    }
    /// PROCESSED PROPS
    pub struct ProcessedProps {
        pub initial_count: u32,
        pub step: u32,
    }
    impl crate::component::ProcessedProps for ProcessedProps {
        type Props = Props;
    }
    impl From<Props> for ProcessedProps {
        fn from(value: Props) -> Self {
            ProcessedProps {
                initial_count: value.initial_count,
                step: Option::unwrap_or_default(value.step),
            }
        }
    }

    // CONTEXT - END

    // STATE VARS - START
    type StateRefCell<const INDEX: usize> = crate::state::StateRefCell<
        {
            use crate::component::Scope as _;
            Scope::N_TEXT_NODES
        },
        crate::state::StateBase<
            INDEX,
            {
                use crate::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            Scope,
        >,
        Scope,
        u32,
    >;
    type StateBase<const INDEX: usize> = crate::state::StateBase<
        INDEX,
        {
            use crate::component::Scope as _;
            Scope::N_TEXT_NODES
        },
        Scope,
    >;
    // 1 - START
    impl
        crate::state::State<
            {
                use crate::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            Scope,
        > for StateBase<0>
    {
        fn on_update(cx: &Scope) {
            // REPEAT THIS LINE FOR EVERY TEXT NODE THAT DEPPENDS ON THIS STATE VAR
            crate::component::Scope::update_text_node::<0>(cx);
            // ADD MORE PRESET EFFECTS (lifecycle hooks, maybe more)
        }
    }
    // 1 - END
    // STATE VARS - END

    // TEXT NODES - START
    // 1 - START
    impl
        crate::component::WithTextNode<
            0,
            {
                use crate::component::Scope as _;
                Scope::N_TEXT_NODES
            },
        > for Context
    {
        fn get_text_node_data(&self) -> String {
            let cx = self;
            {
                let val = { cx.multiplied(3) };
                crate::component::ToData::to_data(&val)
            }
        }
    }
    // 1 - END
    // TEXT NODES - END

    // SCOPE - START
    type Scope = crate::component::ScopeBase<1, Context>;

    impl crate::component::WithProps for Scope {
        type Props = Props;

        // TODO implementation without nightly (#![feature(new_uninit)]) should be possible
        fn new(props: Self::Props) -> ::std::rc::Rc<Self> {
            let ProcessedProps {
                initial_count,
                step,
            } = crate::component::Props::process(props);

            let State { count } = init_state(&initial_count, &step);

            let mut context: ::core::mem::MaybeUninit<Context> = ::core::mem::MaybeUninit::uninit();
            let context_ptr: *mut Context = context.as_mut_ptr();
            unsafe {
                ::core::ptr::addr_of_mut!((*context_ptr).initial_count).write(initial_count);
            }
            unsafe {
                ::core::ptr::addr_of_mut!((*context_ptr).step).write(step);
            }
            let mut scope: ::std::rc::Rc<::core::mem::MaybeUninit<Self>> =
                ::std::rc::Rc::new_uninit();

            let scope_ptr: *mut Scope = ::std::rc::Rc::get_mut(&mut scope).unwrap().as_mut_ptr();

            let count: StateRefCell<0> = {
                let context = scope.clone();
                let context = unsafe { ::std::mem::transmute(context) };
                StateRefCell::new(count, context)
            };

            unsafe {
                ::core::ptr::addr_of_mut!((*context_ptr).count).write(count);
            }

            let context = unsafe { context.assume_init() };

            let text_nodes: [web_sys::Text; {
                use crate::component::Scope as _;
                Self::N_TEXT_NODES
            }] = rapyd_macros::arr!(|I, 1| {
                web_sys::Text::new_with_data(&crate::component::WithTextNode::<
                    I,
                    {
                        use crate::component::Scope as _;
                        Self::N_TEXT_NODES
                    },
                >::get_text_node_data(&context))
                .unwrap()
            });

            unsafe {
                ::core::ptr::addr_of_mut!((*scope_ptr).cx).write(context);
            }

            unsafe {
                ::core::ptr::addr_of_mut!((*scope_ptr).text_nodes).write(text_nodes);
            }

            unsafe { scope.assume_init() }
        }
    }
    // SCOPE - END
    // MACRO GERATED - END
}

fn main() {
    //let test: [usize; 4] = arr!(|I, 4| I);
}
