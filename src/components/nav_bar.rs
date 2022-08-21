use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
            <>
            <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
      <Link<Route> classes={classes!("navbar-item")} to={Route::Fetch}>
        <img src= "https://i.postimg.cc/wvmG7w19/Puss.png" width="112" height="50" />
      </Link<Route>>


        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>

      </div>
        <div class="navbar-end">
        </div>
    </nav>

            </>
        }
}
