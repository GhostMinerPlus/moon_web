use std::io;

use yew::prelude::*;

use crate::service;

async fn fetch_server_v() -> io::Result<Vec<Server>> {
    let server = ["$->$output = = root->web_server _", "server"].join("\\n");
    let name = ["$->$output = = $->$input->name _", "name"].join("\\n");
    let ip = ["$->$output = = $->$input->ip _", "ip"].join("\\n");
    let port = ["$->$output = = $->$input->port _", "port"].join("\\n");
    let rs = service::moon_server_http_execute(&format!(
        r#"{{
"{server}": {{
    "{name}": null,
    "{ip}": null,
    "{port}": null
}}
}}"#
    ))
    .await?;
    let rs = json::parse(&rs).map_err(|e| {
        log::warn!("when fetch_server_v:\n{e}");
        io::Error::other(e)
    })?;

    let mut server_v = Vec::new();
    if rs["server"].is_empty() || !rs["server"].has_key("name") {
        return Ok(server_v);
    }

    for i in 0..rs["server"]["name"].len() {
        let name = rs["server"]["name"][i][0].as_str().unwrap_or_default();
        let ip = rs["server"]["ip"][i][0].as_str().unwrap_or_default();
        let port = rs["server"]["port"][i][0].as_str().unwrap_or_default();
        let uri = format!("{ip}:{port}");
        server_v.push(Server {
            name: name.to_string(),
            service_v: vec![Service {
                protocol: "http".to_string(),
                uri,
            }],
        });
    }

    Ok(server_v)
}

// Public
pub enum Msg {
    Error(String),
    Fetched(Vec<Server>),
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
    server_v: Vec<Server>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match fetch_server_v().await {
                Ok(rs) => Msg::Fetched(rs),
                Err(e) => Msg::Error(e.to_string()),
            }
        });

        Self {
            server_v: Vec::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetched(r) => {
                self.server_v = r;
                true
            }
            Msg::Error(e) => {
                // TODO: 弹窗
                log::error!("when update:\n{e}");
                false
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            {for self.server_v.iter().map(|item| {
                html!(<li>{item.name.clone()}</li>)
            })}
        }
    }
}
