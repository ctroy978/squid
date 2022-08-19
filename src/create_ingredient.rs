use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json;
use web_sys::{HtmlInputElement, Node};
use yew::prelude::*;

use crate::text_box::TextBox;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onclick: Callback<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AddIngredient {
    label: String,
    unit: String,
    qty: String,
}

#[function_component(CreateIngredient)]
pub fn create_ingredient(props: &Props) -> Html {
    let state = use_state(|| AddIngredient::default());

    let cloned_state = state.clone();
    let handle_label = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.label = value;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let handle_unit = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.unit = value;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let handle_qty = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.qty = value;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let handle_on_click = props.handle_onclick.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        //unpack
        let mut data = (*cloned_state).clone();
        let serialize_data = serde_json::to_string(&data).unwrap();
        handle_on_click.emit(serialize_data);
    });

    html! {
        <div>
        <TextBox  name = "label" place_holder = "Enter ingredient name" handle_onchange={handle_label} />
        <TextBox name = "unit" place_holder = "Enter unit of measure (ex. ounce)" handle_onchange={handle_unit} />
        <TextBox name = "qty" place_holder = "How many?" handle_onchange={handle_qty} />
        <br />
        <button onclick = {onclick} class = "button is-success"> {"Add this Ingredient"}</button>
        </div>
    }
}
