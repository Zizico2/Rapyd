use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait Component {
    type ComponentProps: Props;
    type ComponentState: State;
    fn render(props: Self::ComponentProps) -> String;
}
pub trait Props {}
pub trait State: Index<u32> {}

// Concrete

/*
pub struct SampleComponent {}

impl Component for SampleComponent {
    type ComponentProps = SampleProps;
    type ComponentState = SampleState;
    fn render(props: Self::ComponentProps) -> String {
        format!("<div>Hello, {}!</div>", props.name).into()
    }
}

pub struct SampleProps {
    name: String,
}

impl Props for SampleProps {}

pub struct SampleState {}
impl State for SampleState {}

/*
impl IndexMut<u32> for SampleState {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        todo!()
    }
}
*/

impl Index<u32> for SampleState {
    type Output = String;
    fn index(&self, index: u32) -> &Self::Output {
        todo!()
    }
}
*/
/*
fn ttest() {
    fn update_state() -> [String; 3] {
        fn inner_update_state<T1: Display, T2: Display, T3: Display>(
            st_1: T1,
            st_2: T2,
            st_3: T3,
        ) -> [String; 3] {
            [st_1.to_string(), st_2.to_string(), st_3.to_string()]
        }
        {
            let world = "world";
        }
        inner_update_state(world, "", "")
    }
}
 */