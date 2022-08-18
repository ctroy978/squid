use yew::prelude::*;
use yew_router::prelude::*;

pub const SERV_URL: &str = "http://192.168.1.113:8080";

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/fetch")]
    Fetch,
    #[at("/create")]
    Create,
    #[not_found]
    #[at("/404")]
    NotFound,
}

mod components;
mod create_drink;
mod create_ingredient;
mod fetch_drink;

use components::{get_booze, models, nav_bar, post_drink, text_box};
use create_drink::CreateDrink;
use fetch_drink::FetchDrink;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Squid Drinker" }</h1> },
        Route::Fetch => html! {
            <FetchDrink />
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
