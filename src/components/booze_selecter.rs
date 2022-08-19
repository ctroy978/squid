use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub handle_booze_onclick: Callback<String>,
}

#[function_component(BoozeSelecter)]
pub fn booze_selecter(props: &Props) -> Html {
    let handle_booze_onclick = props.handle_booze_onclick.clone();
    let handle_booze = Callback::from(move |e: MouseEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input = input.value();
        handle_booze_onclick.emit(input);
    });
    html! {
        <div>

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

    }
}
