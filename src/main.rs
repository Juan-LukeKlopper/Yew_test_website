use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let header = "Hello world".to_string();
    let counter = use_state(|| 0 );
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{ header }</h1>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
