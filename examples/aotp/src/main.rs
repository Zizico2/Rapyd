use rapyd::rapyd_macros::component;

fn main() {
    println!("Hello, world!");
}

#[component]
mod Counter {
    struct Scope {
        #[prop]
        #[state]
        count: u32,
    }
}
