use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home Page" }</h1>
            <p>{ "Welcome to the homepage." }</p>
        </div>
    }
}