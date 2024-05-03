use std::io;

use wasm_bindgen_futures::JsFuture;

use crate::util;

pub async fn execute(script_tree: json::JsonValue) -> io::Result<json::JsonValue> {
    let res = util::Request::new("/service/moon_server/execute")
        .with_body_txt(&json::stringify(script_tree))?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    Ok(json::parse(&rs).unwrap())
}
