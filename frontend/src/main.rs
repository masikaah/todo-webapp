use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Hello from Yew in a Workspace!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
