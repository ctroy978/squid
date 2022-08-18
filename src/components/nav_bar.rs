use crate::Route;
use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub handle_booze_onclick: Callback<String>,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let handle_booze_onclick = props.handle_booze_onclick.clone();
    let handle_booze = Callback::from(move |e: MouseEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input = input.value();
        handle_booze_onclick.emit(input);
    });
    html! {
            <>
            <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="https://bulma.io">
          <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
        </a>

        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div id="navbarBasicExample" class="navbar-menu">
        <div class="navbar-start">
          <Link<Route>  to={Route::Home} classes={classes!("navbar-item")} >
            {"Home"}
          </Link<Route>>

          <div class="navbar-item has-dropdown is-hoverable">
            <a class="navbar-link">
              {"Choose Booze"}
            </a>

            <div class="navbar-dropdown">
              <button class = "navbar-item button is-text"
              onclick = {&handle_booze} value = {"whiskey"}>
              {"Whiskey"}
              </button>
              <button class = "navbar-item button is-text"
              onclick = {&handle_booze} value = {"rum"}>
              {"Rum"}
              </button>
              <button class = "navbar-item button is-text"
              onclick = {&handle_booze} value = {"gin"}>
              {"Gin"}
              </button>

            </div>
          </div>
        </div>

        <div class="navbar-end">
        </div>
      </div>
    </nav>

            </>
        }
}
