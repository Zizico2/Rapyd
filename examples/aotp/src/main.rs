#![feature(new_uninit)]

//#[component]
mod number_display {
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
    impl
        ::rapyd::component::Context<
            1,
            {
                use ::rapyd::component::Scope as _;
                Scope::N_WALKS
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

    // TEXT NODES - START
    // 1 - START
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
        > for Context
    {
        fn get_text_node_data(&self) -> String {
            let cx = self;
            {
                let val = { cx.multiplied(3) };
                ::rapyd::component::ToData::to_data(&val)
            }
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
        Context,
        Scope,
    >;

    pub struct Scope(ScopeBase);

    impl ::rapyd::component::Scope<1, 5> for Scope {
        type Context = Context;
        type Props = Props;
        const TEMPLATE: &'static str = "<div>I count <!>!</div>";
        const WALKS: [::rapyd::component::Walk; Self::N_WALKS] = [
            ::rapyd::component::Walk::In(1),
            ::rapyd::component::Walk::Event("click"),
            ::rapyd::component::Walk::Over(1),
            ::rapyd::component::Walk::Text,
            ::rapyd::component::Walk::Out(1),
        ];

        fn get_scope_base(
            &self,
        ) -> &rapyd::component::ScopeBase<
            { ScopeBase::N_TEXT_NODES },
            { Self::N_WALKS },
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
                let context = scope.clone();
                let context = unsafe { ::std::mem::transmute(context) };
                StateRefCell::new(count, context)
            };

            unsafe {
                ::core::ptr::addr_of_mut!((*context_ptr).count).write(count);
            }

            let context = unsafe { context.assume_init() };

            let text_nodes: [web_sys::Text; {
                use ::rapyd::component::Scope as _;
                Self::N_TEXT_NODES
            }] = rapyd_macros::arr!(|I, 1| {
                web_sys::Text::new_with_data(&::rapyd::component::WithTextNode::<
                    I,
                    {
                        use ::rapyd::component::Scope as _;
                        Self::N_TEXT_NODES
                    },
                    {
                        use ::rapyd::component::Scope as _;
                        Scope::N_WALKS
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

            unsafe { scope.assume_init() }
        }
    }
    // SCOPE - END
    // MACRO GERATED - END
}

fn main() {
    //let test: [usize; 4] = arr!(|I, 4| I);
}
