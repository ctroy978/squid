use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub place_holder: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextBox)]
pub fn text_box(props: &Props) -> Html {
    let handle_on_change = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();

        handle_on_change.emit(input.value())
    });
    html! {
        <div>
        <input type = "text" name = {props.name.clone()} placeholder = {props.place_holder.clone()}{onchange}/>
        </div>
    }
}
