use gloo::console::log;
use wasm_bindgen_futures;

use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct BuildDrink {
    username: String,
}

#[derive(Serialize, Deserialize)]

pub struct SendBuild {
    username: String,
}

pub enum Msg {
    SetUsername(String),
    DoNothing,
}

impl Component for BuildDrink {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            username: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetUsername(val) => {
                self.username = val;
            }
            Msg::DoNothing => (),
        }
        log!(&self.username);
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onchange = ctx.link().callback(|e: Event| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let value = input_el.value();
            Msg::SetUsername(value)
        });

        let onclick = ctx.link().callback(move |e: MouseEvent| {
            let sendit = SendBuild {
                username: "help".to_string(),
            };
            //post to server
            wasm_bindgen_futures::spawn_local(async move {
                let response = reqwest::Client::new()
                    .post("http://192.168.1.113:8080/build")
                    .json(&sendit)
                    .send()
                    .await;
            });
            Msg::DoNothing
        });

        html! {
            <>
            <div>
                <h1>{"Build That Drink"}</h1>
                <form>
                    <input {onchange} type = "text" name="username" />
                    <button {onclick}>{"Submit"}</button>
                </form>
            </div>
            </>
        }
    }
}
