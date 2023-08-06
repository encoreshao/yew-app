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
            <h1>
                {"Rust with Yew - "}
                <code>
                    {"Counter"}
                </code>
            </h1>
            <button class="pulse" {onclick}>{ "+1" }</button>
            <h1>{ *counter }</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
