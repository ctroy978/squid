use gloo::console::log;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    SetTitle(String),
    SendDrink,
    DoNothing,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BuildDrink {
    title: String,
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

            Msg::SendDrink => {
                let data_serialized = serde_json::to_string_pretty(&self).unwrap();
                log!(&data_serialized);
                post_server(data_serialized);
                //go get api
                /*
                wasm_bindgen_futures::spawn_local(async move {
                    let res = reqwest::Client::new()
                        .post("https://192.168.1.113/drink")
                        .json(&data_serialized)
                        .send()
                        .await;
                });
                */
            }
            Msg::DoNothing => (),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_title = ctx.link().callback(move |e: Event| {
            let input_title: HtmlInputElement = e.target_unchecked_into();
            Msg::SetTitle(input_title.value())
        });

        let send_drink = ctx.link().callback(move |e: MouseEvent| Msg::SendDrink);

        html! {
            <div>
                <h1>{"build drink"}</h1>
                <input type="text" placeholder = "Drink name? " onchange = {get_title} />
                <button onclick = {send_drink}>{"Submit"}</button>
            </div>
        }
    }
}

fn post_server(data: String) {
    //rebuild struct for reqwest
    let str_data = data.as_str();
    let build_now: BuildDrink = serde_json::from_str(str_data).unwrap();
    wasm_bindgen_futures::spawn_local(async move {
        let res = reqwest::Client::new()
            .post("http://192.168.1.113:8080/build")
            .json(&build_now)
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
