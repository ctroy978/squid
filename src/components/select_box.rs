use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange: Callback<String>,
}

#[function_component(SelectBox)]
pub fn select_box(props: &Props) -> Html {
    let handle_on_change = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();

        handle_on_change.emit(input.value())
    });
    html! {
        <div>
        <select  onchange = {onchange} >
            <option value = {"whiskey"}>{"Whiskey"}</option>
            <option value = {"gin"}>{"Gin"}</option>
            <option value = {"rum"}>{"Rum"}</option>
            <option value = {"brandy"}>{"Brandy"}</option>
            <option value = {"genever"}>{"Genever"}</option>
            <option value = {"vodka"}>{"Vodka"}</option>
            <option value = {"taquila"}>{"Taquila"}</option>
            <option value = {"congnac"}>{"Congnac"}</option>
            <option value = {"other"}>{"Other"}</option>
            <option value = {"other"}>{"Choose the booze"}</option>
        </select>

        </div>
    }
}
