use yew::prelude::*;
use stylist::{yew::styled_component, style };

#[styled_component]
fn App() -> Html {
    let header = "Hello world".to_string();

    let stylesheet = style!(
            r#"
                background-color: blue; 
            "#
        ).expect("Failed to load stylesheet");

    html! {
        <html class={stylesheet}>
            <div>
                <h1>{ header }</h1>
                <p>{ "This is a paragraph" }</p>
            </div>
        </html>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
