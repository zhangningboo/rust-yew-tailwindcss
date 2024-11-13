use yew::prelude::*;

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html!(
        <div class="border-t border-slate-400/20 px-3.5 py-3 flex items-center rounded-md p-1.5">
            <h1 class="text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white">
                {"Hello World--"}
            </h1>
        </div>
    )
}

fn main() {
    yew::start_app::<HelloWorld>();
}