use gloo::console::log;
use reqwest::get;
use serde_json::Value;
use wasm_bindgen_futures;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Drink {
    title: String,
    rank: i32,
    directions: String,
    ingredients: Vec<String>,
}

impl Default for Drink {
    fn default() -> Self {
        Self {
            title: String::from("Nothing to drink yet."),
            rank: 0,
            directions: String::from("Still waiting for directions"),
            ingredients: Vec::new(),
        }
    }
}

#[function_component(FetchDrink)]
pub fn fetch_drink() -> Html {
    let drink_state = use_state::<Option<Drink>, _>(|| None);
    let drink_state_out = drink_state.clone();

    let onclick = Callback::from(move |_e: MouseEvent| {
        let drink_state = drink_state.clone();

        //go get api
        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get("http://192.168.1.113:8080/drink/Manhattan")
                .await
                .unwrap();

            let text = response.text().await.unwrap();

            let v: Value = serde_json::from_str(&text).unwrap_or_default();

            let title = v["title"].as_str().unwrap();
            let directions = v["directions"].as_str().unwrap();
            let rank = v["rank"].as_f64().unwrap();

            //only way I know how to pull Values out of a json Vec.
            let ingredients_array = v["ingredients"].as_array().unwrap();
            let mut ingredients = Vec::new();
            for item in ingredients_array.iter() {
                ingredients.push(item.as_str().unwrap().to_owned());
            }

            let drink = Drink {
                title: title.into(),
                rank: rank as i32,
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

    html! {
        <div>
            <button {onclick}>{"get drink"}</button>
            <p>{full_drink.title}</p>
            <p>{full_drink.rank}</p>
            <ul>{ingredients_li}</ul>
            <p>{full_drink.directions}</p>

        </div>
    }
}
