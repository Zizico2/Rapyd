#![feature(new_uninit)]
use rapyd::component::{Scope, WithProps};

//#[component]
mod number_display {

    // USER WRITTEN - START
    use web_sys::Event;

    pub struct Context {
        //#[prop]
        initial_count: u32,
        //#[prop(default)]
        step: u32,
        //#[state]
        count: StateRefCell<0>,
    }

    fn init_state(initial_count: &u32, _step: &u32) -> State {
        State {
            count: *initial_count,
        }
    }
    impl Context {
        fn _multiplied(&self, factor: u32) -> u32 {
            *self.count.borrow() * factor
        }

        fn increment_counter(&self, _: &Event) {
            *self.count.borrow_mut() += self.step;
        }
    }

    /*
    const TEMPLATE: &'static str = "<button>I count <!>!</button>";
    const WALKS: [::rapyd::component::Walk; Self::N_WALKS] = [
        ::rapyd::component::Walk::Event("click"),
        ::rapyd::component::Walk::In(1),
        ::rapyd::component::Walk::Over(2),
        ::rapyd::component::Walk::Text,
        ::rapyd::component::Walk::Over(1),
        ::rapyd::component::Walk::Out(1),
    ];
    html! {
        <div> "I count "{ cx.count }"!" </div>
    };
    */

    // USER WRITTEN - END

    // MACRO GERATED - START

    // CONTEXT - START
    impl
        ::rapyd::component::Context<
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
        > for Context
    {
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
    impl ::rapyd::component::Props for Props {
        type ProcessedProps = ProcessedProps;
    }
    /// PROCESSED PROPS
    pub struct ProcessedProps {
        pub initial_count: u32,
        pub step: u32,
    }
    impl ::rapyd::component::ProcessedProps for ProcessedProps {
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
    type StateRefCell<const INDEX: usize> = ::rapyd::state::StateRefCell<
        {
            use ::rapyd::component::Scope as _;
            Scope::N_TEXT_NODES
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_WALKS
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_EVENT_LISTENERS
        },
        ::rapyd::state::StateBase<
            INDEX,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
            Scope,
        >,
        Scope,
        u32,
    >;
    type StateBase<const INDEX: usize> = ::rapyd::state::StateBase<
        INDEX,
        {
            use ::rapyd::component::Scope as _;
            Scope::N_TEXT_NODES
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_WALKS
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_EVENT_LISTENERS
        },
        Scope,
    >;
    // 1 - START
    impl
        ::rapyd::state::State<
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
            Scope,
        > for StateBase<0>
    {
        fn on_update(cx: &Scope) {
            // REPEAT THIS LINE FOR EVERY TEXT NODE THAT DEPPENDS ON THIS STATE VAR
            ::rapyd::component::Scope::update_text_node::<0>(cx);
            // ADD MORE PRESET EFFECTS (lifecycle hooks, maybe more)
        }
    }
    // 1 - END
    // STATE VARS - END

    // EVENT HANDLERS - START
    // 0 - START
    impl
        ::rapyd::component::WithEventHandler<
            0,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
        > for Context
    {
        fn get_event_handler(
            scope: ::std::rc::Rc<Self::Scope>,
        ) -> wasm_bindgen::closure::Closure<dyn Fn(&web_sys::Event)> {
            use rapyd::component::Scope;
            let scope = scope.clone();
            wasm_bindgen::closure::Closure::wrap(Box::new(move |ev| {
                scope.get_scope_base().cx.increment_counter(ev);
            }))
        }
    }
    // 0 - END
    // 1 - START
    impl
        ::rapyd::component::WithEventHandler<
            1,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
        > for Context
    {
        fn get_event_handler(
            scope: ::std::rc::Rc<Self::Scope>,
        ) -> wasm_bindgen::closure::Closure<dyn Fn(&web_sys::Event)> {
            use rapyd::component::Scope;
            let scope = scope.clone();
            wasm_bindgen::closure::Closure::wrap(Box::new(move |ev| {
                scope.get_scope_base().cx.increment_counter(ev);
            }))
        }
    }
    // 1 - END
    // EVENT HANDLERS - END

    // TEXT NODES - START
    // 0 - START
    impl
        ::rapyd::component::WithTextNode<
            0,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
        > for Context
    {
        fn get_text_node_data(&self) -> String {
            let cx = self;

            ::rapyd::component::ToData::to_data({
                /*{ cx.multiplied(self.step) }*/
                &cx.count
            })
        }
    }
    // 0 - END
    // 1 - START
    impl
        ::rapyd::component::WithTextNode<
            1,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_TEXT_NODES
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
            },
            {
                use ::rapyd::component::Scope as _;
                Scope::N_EVENT_LISTENERS
            },
        > for Context
    {
        fn get_text_node_data(&self) -> String {
            let cx = self;

            ::rapyd::component::ToData::to_data({
                /*{ cx.multiplied(self.step) }*/
                &cx.count
            })
        }
    }
    // 1 - END
    // TEXT NODES - END

    // SCOPE - START
    type ScopeBase = ::rapyd::component::ScopeBase<
        {
            use ::rapyd::component::Scope as _;
            Scope::N_TEXT_NODES
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_WALKS
        },
        {
            use ::rapyd::component::Scope as _;
            Scope::N_EVENT_LISTENERS
        },
        Context,
        Scope,
    >;

    pub struct Scope(ScopeBase);

    impl ::rapyd::component::Scope<2, 11, 2> for Scope {
        type Context = Context;
        type Props = Props;

        const TEMPLATE: &'static str =
            "<div><button>I count <!> clicks</button><button>I count <!> clicks</button></div>";

        rapyd_macros::html_iter! {
            <div><button>"I count "{ cx.count }" clicks"</button><button>"I count "{ cx.count }" clicks"</button></div>
        }

        fn get_scope_base(
            &self,
        ) -> &rapyd::component::ScopeBase<
            { Self::N_TEXT_NODES },
            { Self::N_WALKS },
            { Self::N_EVENT_LISTENERS },
            Self::Context,
            Self,
        > {
            &self.0
        }
    }

    impl ::rapyd::component::WithProps for Scope {
        type Props = Props;

        // TODO implementation without nightly (#![feature(new_uninit)]) should be possible
        fn new(props: Self::Props) -> ::std::rc::Rc<Self> {
            let ProcessedProps {
                initial_count,
                step,
            } = ::rapyd::component::Props::process(props);

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
                let scope = scope.clone();
                let scope = unsafe { ::std::mem::transmute(scope) };
                StateRefCell::new(count, scope)
            };

            unsafe {
                ::core::ptr::addr_of_mut!((*context_ptr).count).write(count);
            }

            let context = unsafe { context.assume_init() };

            let text_nodes: [web_sys::Text; {
                use ::rapyd::component::Scope as _;
                Self::N_TEXT_NODES
            }] = util_macros::arr!(|I, 2| {
                web_sys::Text::new_with_data(&::rapyd::component::WithTextNode::<
                    I,
                    {
                        use ::rapyd::component::Scope as _;
                        Self::N_TEXT_NODES
                    },
                    {
                        use ::rapyd::component::Scope as _;
                        Self::N_WALKS
                    },
                    {
                        use ::rapyd::component::Scope as _;
                        Self::N_EVENT_LISTENERS
                    },
                >::get_text_node_data(&context))
                .unwrap()
            });

            unsafe {
                ::core::ptr::addr_of_mut!((*scope_ptr).0.cx).write(context);
            }

            unsafe {
                ::core::ptr::addr_of_mut!((*scope_ptr).0.text_nodes).write(text_nodes);
            }
            let event_handlers: [::wasm_bindgen::prelude::Closure<dyn Fn(&web_sys::Event)>; 2] = {
                use rapyd::component::Scope as _;

                let scope: &::std::rc::Rc<Scope> = unsafe { ::std::mem::transmute(&scope) };
                util_macros::arr!(|I, 2| {
                    let scope = scope.clone();
                    scope.get_event_handler::<I>()
                })
            };

            unsafe {
                ::core::ptr::addr_of_mut!((*scope_ptr).0.event_handlers).write(event_handlers);
            }

            unsafe { scope.assume_init() }
        }
    }
    // SCOPE - END
    // MACRO GERATED - END
}

fn main() {
    let app = gloo_utils::document()
        .query_selector("#app")
        .unwrap()
        .unwrap();

    number_display::Scope::new(number_display::Props {
        initial_count: 0,
        step: Some(1),
    })
    .render(&app);
}
