use yew::prelude::*;
use yew::{classes, html};

fn main() {
    println!("Starting yew app");
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <div>
            <Producer/>
            <Consumer/>
        </div>
    )
}

#[function_component(Producer)]
fn producer() -> Html {
    html!(
    <div class={classes!("producer")}>
        <h3>
            {"Producer"}
        </h3>
    </div>
    )
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!(
    <div class="consumer">
        <h3>
            {"Consumer"}
        </h3>
    </div>
    )
}
