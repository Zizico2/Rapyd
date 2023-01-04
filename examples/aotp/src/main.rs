use std::fmt::Display;

use component::TextNodeToData;

mod component;
mod state;
//#[component]
mod number_display {
    use crate::component::Context as _;
    use crate::component::TextNodeToData;
    use std::rc::Rc;

    use crate::component;
    use crate::state;
    use crate::state::StateRefCell;

    struct Context {
        //#[prop]
        initial_count: u32,
        //#[prop(default)]
        step: u32,
        //#[state]
        count: StateRefCell<
            { Self::N_TEXT_NODES },
            state::State<0, { Self::N_TEXT_NODES }, Self>,
            Context,
            u32,
        >,
        text_nodes: [web_sys::Text; Self::N_TEXT_NODES],
    }
    impl component::Context<1> for Context {
        const N_TEXT_NODES: usize = 1;
        fn get_text_nodes(&self) -> &[web_sys::Text; Self::N_TEXT_NODES] {
            &self.text_nodes
        }
    }

    // REPEAT FOR EVERY STATE VAR
    impl state::OnUpdate<{ Context::N_TEXT_NODES }, Context>
        for state::State<0, { Context::N_TEXT_NODES }, Context>
    {
        fn on_update(cx: Rc<Context>) {
            // REPEAT THIS LINE FOR EVERY TEXT NODE THAT DEPPENDS ON THIS STATE VAR
            component::update_text_node::<0, { Context::N_TEXT_NODES }, _>(cx);

            // ADD MORE PRESET EFFECTS (lifecycle hooks, maybe more)
        }
    }

    // REPEAT FOR EVERY TEXT NODE
    impl component::TextNodeToData<Context>
        for component::TextNode<0, { Context::N_TEXT_NODES }, Context>
    {
        fn to_data(cx: Rc<Context>) -> String {
            let val = { cx.multiplied(3) };
            component::ToData::to_data(&val)
        }
    }

    /*
    fn init_state(initial_count: u32, step: u32) -> State {
        State {
            count: initial_count,
        }
    }
    */

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

impl<T: Display> component::ToData for T {
    fn to_data(&self) -> String {
        self.to_string()
    }
}

fn main() {}
