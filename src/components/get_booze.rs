use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

use wasm_bindgen_futures;

use crate::SERV_URL;

use crate::models::Drink;

pub fn get_booze_order(data: String) {
    //go get api
    let drink_url = format!("{}/search/{}", SERV_URL, data);
    wasm_bindgen_futures::spawn_local(async move {
        let response = reqwest::get(drink_url).await.unwrap();

        let text = response.text().await.unwrap();

        let v: Vec<Value> = serde_json::from_str(&text).unwrap_or_default();
        let mut list_drink: Vec<String> = Vec::new();
        for v_drink in v {
            let title = v_drink["title"].as_str().unwrap();
            list_drink.push(String::from(title));
        }
    });
}
