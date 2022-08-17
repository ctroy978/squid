use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/fetch")]
    Fetch,
    #[at("/build")]
    Build,
    #[at("/create")]
    Create,
    #[not_found]
    #[at("/404")]
    NotFound,
}

mod build_drink;
mod choose_booz;
mod components;
mod create_drink;
mod create_ingredient;
mod fetch_drink;
use build_drink::BuildDrink;
use components::{post_drink, text_box};
use create_drink::CreateDrink;
use fetch_drink::FetchDrink;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Squid Drinker" }</h1> },
        Route::Fetch => html! {
            <FetchDrink />
        },
        Route::Build => html! {
            <BuildDrink />
        },
        Route::Create => html! {
            <CreateDrink />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
