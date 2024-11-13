use yew::prelude::*;

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html!(
        <h1 class="text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white">
            {"Hello World"}
        </h1>
    )
}

fn main() {
    yew::start_app::<HelloWorld>();
}