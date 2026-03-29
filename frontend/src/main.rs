use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Home, About, Task}; // Import the pages


mod pages; // import the pages module

// Define the routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/task")]
    Task,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Switch component that renders the correct page based on the route
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
        Route::Task => html! { <Task /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div>
                <nav>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                    <Link<Route> to={Route::Task}>{ "Task" }</Link<Route>>
                </nav>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}