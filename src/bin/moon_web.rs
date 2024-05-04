use log::Level;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use moon_web::{
    component::{self, menu::Menu},
    router::{self, Route},
};

struct App {
    base_url: String,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let base_url = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .base_uri()
            .unwrap()
            .unwrap();
        Self { base_url }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut tree = component::menu::Node::new();
        tree.insert("home".to_string());
        tree.insert("404".to_string());

        let base_url = self.base_url.clone();
        let menu_switch = {
            Callback::from(move |key: String| {
                let location = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .location()
                    .unwrap();
                let _ = match key.as_str() {
                    "home" => location.replace(&format!("{base_url}")),
                    _ => location.replace(&format!("{base_url}404")),
                };
            })
        };

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Moon"}</div>
                <div class={"main-content"}>
                    <Menu {tree} switch={menu_switch} />
                    <BrowserRouter><Switch<Route> render={router::switch} /></BrowserRouter>
                </div>
            </div>
        }
    }
}

fn main() {
    let _ = console_log::init_with_level(Level::Info);
    yew::Renderer::<App>::new().render();
}
