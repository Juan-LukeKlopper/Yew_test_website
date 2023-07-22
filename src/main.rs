use yew::prelude::*;
use stylist::{yew::styled_component, style };

#[styled_component]
fn App() -> Html {
    let header = "Coming Soon!".to_string();
    let paragraph = "Please Stand by as we're getting ready for Blast off!".to_string();

    let stylesheet = style!(
            r#"
                background-color: None;
                padding-top: max(100px, 12.5%);
                padding-right: 7.5%;
                padding-bottom: 2%;
                padding-left: 7.5%;
                "#
        ).expect("Failed to load stylesheet");

    html! {
        <div class={stylesheet}>
            <div>
                <h1>{ header }</h1>
                <p>{ paragraph }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
