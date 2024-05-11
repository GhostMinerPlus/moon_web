use wasm_bindgen_futures::JsFuture;

use crate::{err, util};

pub async fn moon_server_http_execute(script_tree: &str) -> err::Result<String> {
    let res = util::Request::new("/service/moon_server/execute")
        .with_body_txt(script_tree)?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(err::Error::Other("returned empty".to_string()))?;
    Ok(rs)
}
