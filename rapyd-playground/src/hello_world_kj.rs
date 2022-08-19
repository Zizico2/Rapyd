use rapyd_macros::{component, do_nothing};

component! {
    <script>
        {
            let one = 1;
            let three = 3;
            let hello = "Hello";
            let world = "World!";
            let test = || {
                three + one
            };
            let hello_world = || {
                format!("{} {}", hello, world)
            };
        }
    </script>

    <template>
        <section>
            <span></span>
            <div>
                "disab dsi"
            </div>
            <div>
                { test() }
                <div>
                    { test() * 4 }
                </div>
            </div>
            <!-- "dsadds" -->
            <div>
                { hello_world() }
            </div>
        </section>
    </template>
}

// HTML TAGS RORDER IS STILL WONG
