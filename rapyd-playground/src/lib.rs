use enclose::enclose;
use rapyd_macros::always_works;
use rapyd_macros::{always_works_attr, function_component};

#[function_component]
pub fn my_func(age: i32, name: String) {
    let mut count: u32 = state!(0);
    let another: String = state!("".to_string());
    //let aq = count.borrow_mut();

    let count_1 = count.clone();
    let count_2 = count.clone();

    let click_handler = enclose!((mut count) move || {
        *count.borrow_mut() += 1;
    });

    let click_handler_2 = enclose!((mut count) move || {
        *count.borrow_mut() += 1;
    });
    render! (
        <div>
            <button on:click={click_handler}>
                "debug i"
            </button>
            <button on:click={click_handler_2}>
                "debug i"
            </button>
            <button>
                <span>
                    { *count_1.borrow() + *count_2.borrow() }
                    "Some Text"
                </span>
            </button>
            <button>
            </button>
        </div>
    )
}
