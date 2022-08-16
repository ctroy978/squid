use std::ops::Add;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::create_ingredient::CreateIngredient;
use crate::text_box::TextBox;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct AddIngredients {
    label: String,
    unit: String,
    qty: String,
}

impl std::fmt::Display for AddIngredients {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {} {} of {})", self.qty, self.unit, self.label)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct BuildDrink {
    pub title: String,
    pub rank: String,
    pub directions: String,
    pub add_ingredient: Vec<AddIngredients>,
}

#[function_component(CreateDrink)]
pub fn create_drink() -> Html {
    let state = use_state(|| BuildDrink::default());

    //handle title
    let cloned_state = state.clone();
    let handle_title = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.title = value;
        cloned_state.set(data);
    });

    //handle rank
    let cloned_state = state.clone();
    let handle_rank = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.rank = value;
        cloned_state.set(data);
    });

    //handle directions
    let cloned_state = state.clone();
    let handle_dir = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.directions = value;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let handle_ingredient = Callback::from(move |value: String| {
        let mut data = (*cloned_state).clone();
        let add_ingredients: AddIngredients = serde_json::from_str(&value.as_str()).unwrap();
        data.add_ingredient.push(add_ingredients);
        cloned_state.set(data);
    });

    let ingredient_list = (*state.add_ingredient)
        .iter()
        .map(|ing| html! {<li>{ing}</li>});

    html! {

        <>
        <h1>{"drink comp"}</h1>

        <TextBox name = "title"  place_holder = {"Drink's name?"} handle_onchange = {handle_title}/>
        <TextBox name = "rank"  place_holder = {"Drink's rank?"} handle_onchange = {handle_rank}/>
        <TextBox name = "directions"  place_holder = {"Step by step directions?"} handle_onchange = {handle_dir}/>
        <br />
        <CreateIngredient handle_onclick = {handle_ingredient}/>
        <div>
            <p>{"Drink name: "}{&state.title}</p>
            <p>{"Rank: "}{&state.rank}</p>
            <p>{"Directions: "}{&state.directions}</p>

            <ul>
                {for ingredient_list}
            </ul>
        </div>



        </>
    }
}
