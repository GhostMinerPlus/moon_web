use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

use crate::util::Request;

pub enum Msg {
    Fetched(String),
}

pub struct Home {
    content: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let r = JsFuture::from(
                Request::new("/moon/service/get?name=edge")
                    .send("GET")
                    .await
                    .unwrap()
                    .text()
                    .unwrap(),
            )
            .await
            .unwrap()
            .as_string()
            .unwrap();
            Msg::Fetched(r)
        });

        Self {
            content: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetched(r) => {
                self.content = r;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ &self.content }</h1>
        }
    }
}
