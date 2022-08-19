use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub place_holder: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let handle_on_change = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();

        handle_on_change.emit(input.value())
    });
    html! {
        <div>
        <textarea rows = "5" name = {props.name.clone()} placeholder = {props.place_holder.clone()}{onchange}></textarea>
        </div>
    }
}
