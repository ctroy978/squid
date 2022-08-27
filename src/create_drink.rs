use gloo::console::log;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::components::{
    nav_bar::NavBar, post_drink::post_server, select_box::SelectBox, text_area::TextArea,
    text_box::TextBox,
};
use crate::create_ingredient::CreateIngredient;
use crate::Route;

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
    pub booz: String,
    pub directions: String,
    pub add_ingredient: Vec<AddIngredients>,
}

#[function_component(CreateDrink)]
pub fn create_drink() -> Html {
    //node ref is for post button

    let state = use_state(|| BuildDrink::default());

    //handle title
    let cloned_state = state.clone();
    let handle_title = Callback::from(move |value: String| {
        let trim_and_lower = &value.trim().to_lowercase();
        let mut data = (*cloned_state).clone();
        data.title = trim_and_lower.to_owned();
        cloned_state.set(data);
    });

    //handle rank
    let cloned_state = state.clone();
    let handle_rank = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.rank = value;
        cloned_state.set(data);
    });

    //handle booz
    let cloned_state = state.clone();
    let handle_booz = Callback::from(move |value: String| {
        let trim_and_lower = &value.trim().to_lowercase();
        let mut data = (*cloned_state).clone();
        data.booz = trim_and_lower.to_owned();
        cloned_state.set(data);
    });

    //handle directions
    let cloned_state = state.clone();
    let handle_dir = Callback::from(move |value| {
        let mut data = (*cloned_state).clone();
        data.directions = value;
        cloned_state.set(data);
    });

    //handle ingredients
    let cloned_state = state.clone();
    let handle_ingredient = Callback::from(move |value: String| {
        let mut data = (*cloned_state).clone();
        let add_ingredients: AddIngredients = serde_json::from_str(&value.as_str()).unwrap();
        data.add_ingredient.push(add_ingredients);
        cloned_state.set(data);
    });

    //handle ingredient removal
    let cloned_state = state.clone();
    let handle_ingredient_removal = Callback::from(move |e: MouseEvent| {
        let mut data = (*cloned_state).clone();
        let input: HtmlInputElement = e.target_unchecked_into();
        let list_number = input.value().parse().unwrap();
        data.add_ingredient.remove(list_number);
        cloned_state.set(data);
    });

    let ingredient_list = (*state.add_ingredient)
        .iter()
        .enumerate()
        .map(|(num, ing)| {
            let list_pos: String = num.to_string();
            html! {
                <li>
                 <div>
                    {ing}{" "}<button value = {list_pos}
                    class = "button is-small is-rounded is-warning is-hoovered"
                    onclick = {handle_ingredient_removal.clone()}>{"remove"}</button>
                 </div>
                </li>
            }
        });

    //handle upload
    let cloned_state = state.clone();
    let history = use_history().unwrap();
    let post_drink = Callback::from(move |e: MouseEvent| {
        let data = (*cloned_state).clone();
        let serial_data = serde_json::to_string(&data).unwrap();
        post_server(serial_data);
        history.push(Route::Fetch);
    });

    html! {
        <>
        <NavBar />

        <section class="section">
            <div class = "containter">
            <h1>{"Let's build a drink"}</h1>
                <div class = "columns">
                    <div class = "column is-3">


                        <TextBox name = "title"  place_holder = {"Drink's name?"} handle_onchange = {handle_title}/>
                        <TextBox name = "rank"  place_holder = {"Drink's rank?"} handle_onchange = {handle_rank}/>
                        <div class="select">
                            <SelectBox  handle_onchange = {handle_booz} />
                        </div>
                        <TextArea name = "directions" place_holder = {"Step by step directions"} handle_onchange = {handle_dir} />
                        </div>
                    <div class="column is-3">

                        <CreateIngredient handle_onclick = {handle_ingredient}/>

                    </div>
                    <div class="column is-4">

                        <p>{"Drink name: "}{&state.title}</p>
                        <p>{"Rank: "}{&state.rank}</p>
                        <p>{"Booz: "}{&state.booz}</p>
                        <p>{"Directions: "}{&state.directions}</p>

                        <ul>
                            {for ingredient_list}
                        </ul>
                        <div class = "py-5">
                        <button onclick={post_drink} class = "button is-danger">{"Post drink to server?"}</button>
                        </div>
                    </div>
                    <div class="column is-2">
                    </div>
                    </div>
            </div>
        </section>


        </>
    }
}
