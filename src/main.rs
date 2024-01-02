mod components;
mod router;
mod views;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{components::menu::Menu, router::Route};

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut tree = components::menu::Node::new();
        tree.insert("home".to_string());

        let menu_switch = {
            Callback::from(move |key: String| {
                let location = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .location()
                    .unwrap();
                let _ = match key.as_str() {
                    "home" => location.replace("/"),
                    _ => location.replace("/404"),
                };
            })
        };

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
                    <Menu {tree} switch={menu_switch} />
                    <BrowserRouter><Switch<Route> render={router::switch} /></BrowserRouter>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
