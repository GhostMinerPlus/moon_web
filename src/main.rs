use log::Level;
use yew::prelude::*;

use moon_web::{component, router, util};

struct App {
    base_uri: String,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let base_uri = match util::get_base_uri() {
            Ok(rs) => match rs {
                Some(rs) => rs,
                None => panic!("when create:\n\tfailed to get base uri"),
            },
            Err(e) => panic!("when create:\n{e}"),
        };
        Self { base_uri }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let mut tree = component::menu::Node::new();
        tree.insert("home".to_string());

        let base_url = self.base_uri.clone();
        let menu_switch = Callback::from(move |key: String| {
            let location = match util::get_location() {
                Some(rs) => rs,
                None => {
                    log::warn!("when view\n:\t无法获取 location 实例");
                    return;
                }
            };
            if let Err(e) = match key.as_str() {
                "home" => location.replace(&format!("{base_url}")),
                _ => location.replace(&format!("{base_url}404")),
            } {
                let e = util::map_js_error(e);
                log::warn!("when view\n:\t{e}");
            }
        });

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Moon"}</div>
                <div class={"main-content"}>
                    <component::menu::Menu {tree} switch={menu_switch} />
                    <router::Router />
                </div>
            </div>
        }
    }
}

fn main() {
    let _ = console_log::init_with_level(Level::Info);
    yew::Renderer::<App>::new().render();
}
