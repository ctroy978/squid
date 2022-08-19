use gloo::console::log;
use serde_json::Value;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{booze_selecter::BoozeSelecter, nav_bar::NavBar};
use crate::{Route, SERV_URL};

#[derive(Default, PartialEq, Clone)]
pub struct BoozeSelect {
    booze_select: Vec<String>,
}

#[derive(PartialEq, Clone)]
pub struct Drink {
    title: String,
    rank: i32,
    booz: String,
    directions: String,
    ingredients: Vec<String>,
}

impl Default for Drink {
    fn default() -> Self {
        Self {
            title: String::from("Nothing to drink yet."),
            rank: 0,
            booz: String::from("Core booz for this drink."),
            directions: String::from("Still waiting for directions"),
            ingredients: Vec::new(),
        }
    }
}

#[function_component(FetchDrink)]
pub fn fetch_drink() -> Html {
    //fetch booze list of drinks
    let booze_state = use_state(|| BoozeSelect::default());
    let cloned_booze_state_out = booze_state.clone();

    let handle_booze_select = Callback::from(move |value| {
        let booze_state = booze_state.clone();

        let mut data = (*booze_state).clone();
        data.booze_select.clear();

        //go get api
        let drink_url = format!("{}/search/{}", SERV_URL, value);
        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(drink_url).await.unwrap();

            let text = response.text().await.unwrap();

            let v: Vec<Value> = serde_json::from_str(&text).unwrap_or_default();
            for v_drink in v {
                let title = v_drink["title"].as_str().unwrap();
                data.booze_select.push(String::from(title));
            }
            booze_state.set(data);
        });
    });

    //fetch drink
    let drink_state = use_state::<Option<Drink>, _>(|| None);

    let drink_state_out = drink_state.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        let drink_state = drink_state.clone();
        let input: HtmlInputElement = e.target_unchecked_into();
        let input = input.value();

        //go get api
        let drink_url = format!("{}/drink/{}", SERV_URL, input);
        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(drink_url).await.unwrap();

            let text = response.text().await.unwrap();

            let v: Value = serde_json::from_str(&text).unwrap_or_default();

            let title = v["title"].as_str().unwrap();
            let directions = v["directions"].as_str().unwrap();
            let rank = v["rank"].as_f64().unwrap();
            let booz: &str = v["booz"].as_str().unwrap();

            //only way I know how to pull Values out of a json Vec.
            let ingredients_array = v["ingredients"].as_array().unwrap();
            let mut ingredients = Vec::new();
            for item in ingredients_array.iter() {
                ingredients.push(item.as_str().unwrap().to_owned());
            }

            let drink = Drink {
                title: title.into(),
                rank: rank as i32,
                booz: booz.into(),
                directions: directions.into(),
                ingredients,
            };
            drink_state.set(Some(drink));
        });
    });
    let ds = (*drink_state_out).clone();
    let full_drink = match ds {
        Some(drink) => drink,
        None => Drink::default(),
    };

    let ingredients_li = full_drink
        .ingredients
        .iter()
        .map(|i| {
            html! {<li>{i}</li>}
        })
        .collect::<Html>();

    //build a list of drinks
    let new_booze_state = (*cloned_booze_state_out).clone();
    let list_drinks = new_booze_state.booze_select.iter().map(|title| {
        html! {
            <li>
        <button  onclick = {&onclick} value = {title.to_string()} class = "button is-text is-small">
            {title}
        </button>

        </li>
        }
    });

    html! {
        <div>
            <NavBar/>

            //main
            <section class="section">
            <div class = "containter">
            <div class = "columns">

            <div class = "column is-2">
                <h1 class="is-size-1, title">{"Want it"}</h1>
                <BoozeSelecter handle_booze_onclick = {handle_booze_select} />
            </div>
            <div class = "column is-2">
                <h1 class="is-size-1, title">{"List it"}</h1>
                <ul>{for list_drinks}</ul>
            </div>
            <div class = "column is-3" >
                <h1 class="is-size-1, title">{"Mix it"}</h1>

                <p class="is-size-4">{full_drink.title}</p>
                <p>{"Rank: "}{full_drink.rank}</p>

                <h3 class = "is-size-4">{"Ingredients"}</h3>
                <ul>{ingredients_li}</ul>

                <h3 class="is-size-4">{"Directions"}</h3>
                <p>{full_drink.directions}</p>
            </div>
            <div class = "column is-5">
                <h1 class="is-size-1, title">{"Build it."}</h1>
                <Link<Route> classes={classes!("button")} to={Route::Create}>{ "Build a Drink" }</Link<Route>>

            </div>

            </div>
            </div>
            </section>

        </div>
    }
}
