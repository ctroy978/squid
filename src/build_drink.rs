use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{text_box::TextBox};

pub enum Msg {
    SetTitle(String),
    SetRank(String),
    SetDirections(String),
    SendDrink,
    DoNothing,
}

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
    directions: String,
    add_ingredient: Vec<AddIngredient>,
}

impl Component for BuildDrink {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetTitle(val) => {
                self.title = val;
            }
            Msg::SetRank(val) => {
                self.rank = val;
            }
            Msg::SetDirections(val) => {
                self.directions = val;
            }

            Msg::SendDrink => {
                let data_serialized = serde_json::to_string_pretty(&self).unwrap();
                post_server(data_serialized);
            }
            Msg::DoNothing => (),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_title = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::SetTitle(input.value())
        });

        let get_rank = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::SetRank(input.value())
        });

        let get_dir = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::SetDirections(input.value())
        });

        let send_drink = ctx.link().callback(move |e: MouseEvent| Msg::SendDrink);

        html! {
           
            <div>
                
                <h1>{"build drink"}</h1>

                <input type="text" placeholder = "Drink name? " onchange = {get_title} />
                <input type="text" placeholder = "Drink Rank? " onchange = {get_rank} />
                <input type="text" placeholder = "Step by Step directions? " onchange = {get_dir} />
                <div>
                    <label>{"Add ingredients"}</label>

                </div>
                <button onclick = {send_drink}>{"Submit finished Drink"}</button>
            </div>
        }
    }
}

fn post_server(data: String) {
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

        let t = res.text().await;
        match t {
            Ok(val) => log!(val),
            Err(e) => log!(format!("error {}", e)),
        }
    });
}
