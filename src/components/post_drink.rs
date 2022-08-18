use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AddIngredient {
    label: String,
    unit: String,
    qty: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BuildDrink {
    title: String,
    rank: String,
    booz: String,
    directions: String,
    add_ingredient: Vec<AddIngredient>,
}

pub fn post_server(data: String) {
    //rebuild struct for reqwest
    let str_data = data.as_str();
    let build_drink: BuildDrink = serde_json::from_str(str_data).unwrap();
    wasm_bindgen_futures::spawn_local(async move {
        let res = reqwest::Client::new()
            .post("http://192.168.1.113:8080/build")
            .json(&build_drink)
            .send()
            .await
            .unwrap();

        let returned_value = res.text().await;
    });
}
