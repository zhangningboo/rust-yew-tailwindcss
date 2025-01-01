use yew::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::*;
use web_sys::{window};

pub mod components;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let _document = window()
        .expect_throw("window is undefined")
        .document()
        .expect_throw("document is undefined");

    html! {
        <>
            <div>
                <h1 class="text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white">
                    {"Hello World"}
                </h1>
                <button { onclick } class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"> { "Click +1" } </button>
                <p> { *counter } </p>
                <table class="border-separate border border-green-800">
                    <thead>
                        <tr>
                        <th class={classes!("border", "border-green-600")}>{ "State" }</th>
                        <th class={classes!("border", "border-green-600")}>{ "City" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                        <td class="border border-green-600">{ "Indiana" }</td>
                        <td class="border border-green-600">{ "Indianapolis" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div>
                <footer> { "Copyright ZNB" } </footer>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}