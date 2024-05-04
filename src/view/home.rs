use yew::prelude::*;

use crate::service;

pub enum Msg {
    Error(String),
    Fetched(String),
}

pub struct Service {
    pub protocol: String,
    pub uri: String,
}

pub struct Server {
    pub name: String,
    pub service_v: Vec<Service>,
}

pub struct Home {
    content: String,
    server_v: Vec<Server>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let server = ["$->$output = = root->server _", "server"].join("\\n");
            let name = ["$->$output = = $->$input->name _", "name"].join("\\n");
            let ip = ["$->$output = = $->$input->ip _", "ip"].join("\\n");
            let port = ["$->$output = = $->$input->port _", "port"].join("\\n");
            let rs = match service::moon_server_http_execute(&format!(
                r#"{{
    "{server}": {{
        "{name}": null,
        "{ip}": null,
        "{port}": null
    }}
}}"#
            ))
            .await
            {
                Ok(rs) => rs,
                Err(e) => {
                    log::warn!("when create:\n {e}");
                    return Msg::Error(e.to_string());
                }
            };
            Msg::Fetched(rs)
        });

        Self {
            content: "".to_string(),
            server_v: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetched(r) => {
                self.content = r;
                true
            }
            Msg::Error(e) => {
                // TODO: 弹窗
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ &self.content }</h1>
        }
    }
}
