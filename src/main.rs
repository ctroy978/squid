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
    #[not_found]
    #[at("/404")]
    NotFound,
}

mod build_drink;
mod fetch_drink;
use build_drink::BuildDrink;
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
