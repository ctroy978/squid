use gloo::console::log;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct BuildDrink {
    username: String,
}

pub enum Msg {
    SetUsername(String),
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
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let value = input_el.value();
            log!(value);
            Msg::SetUsername("tada".to_string())
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
