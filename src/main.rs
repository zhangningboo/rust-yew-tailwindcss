use yew::prelude::*;

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

    html! {
        <div>
            <h1 class="text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white">
                {"Hello World"}
            </h1>
            <button { onclick } class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"> { "Click +1" } </button>
            <p> { *counter } </p>
            <table class="border-separate border border-green-800">
                <thead>
                    <tr>
                    <th class="border border-green-600">{ "State" }</th>
                    <th class="border border-green-600">{ "City" }</th>
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
    }
    // html! {
    //     <table class="border-separate border border-green-800">
    //         <thead>
    //             <tr>
    //             <th class="border border-green-600">{ "State" }</th>
    //             <th class="border border-green-600">{ "City" }</th>
    //             </tr>
    //         </thead>
    //         <tbody>
    //             <tr>
    //             <td class="border border-green-600">{ "Indiana" }</td>
    //             <td class="border border-green-600">{ "Indianapolis" }</td>
    //             </tr>
    //         </tbody>
    //     </table>
    // }
}

fn main() {
    yew::Renderer::<App>::new().render();
}