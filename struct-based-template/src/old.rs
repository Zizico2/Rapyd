/*
use rapyd_macros::{borrow, struct_component, struct_component_impl};
/*
impl Test {
    pub fn new(initial_count: u32, step: Option<u32>) {
        let step = step.unwrap_or_default();
    }
    pub fn apply_step(step: u32) {}
    pub fn apply_initial_count(initial_count: u32) {}
    pub fn add_text_nodes(nodes: slice::Iter<&Text>) {}
    pub fn add_event_targets(nodes: slice::Iter<&EventTarget>) {}
}
*/

#[struct_component]
struct Counter {
    #[state]
    count: u32,
    #[state]
    message: String,
}

#[struct_component_impl]
impl ___SerializableScope {
    fn render(&self) -> String {
        template!(
            <div>
                <div>
                    { borrow!(count) + borrow!(count) }
                </div>
                <div>
                    { format!("{}-{}", borrow!(message), borrow!(message)) }
                </div>
            </div>
        )
    }
    borrow! {}
}
*/
