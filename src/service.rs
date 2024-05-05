use std::io;

use wasm_bindgen_futures::JsFuture;

use crate::util;

pub async fn moon_server_http_execute(script_tree: &str) -> io::Result<String> {
    let res = util::Request::new("/service/moon_server/execute")
        .with_body_txt(script_tree)?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    Ok(rs)
}
