use serde::*;
use serde_wasm_bindgen::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use web_sys::{Request, RequestInit};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserForFetch {
    pub name: String,
    pub pass: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FetchResponse {
    token: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegReturn {
    pub key: String,
    pub token: String,
}
pub enum FetchError {
    JsValue(JsValue),
    Error(Error),
    SerdeError(serde_json::Error),
}
impl From<serde_json::Error> for FetchError {
    fn from(value: serde_json::Error) -> Self {
        FetchError::SerdeError(value)
    }
}
impl From<Error> for FetchError {
    fn from(value: Error) -> Self {
        FetchError::Error(value)
    }
}
impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError::JsValue(value)
    }
}

pub async fn send_register(name: String, pass: String) -> Result<RegReturn, FetchError> {
    let new_user = UserForFetch { name, pass };
    let mut opts = RequestInit::new();
    let str_body = serde_json::to_string(&new_user)?;
    let body = serde_wasm_bindgen::to_value(&str_body)?;

    opts.body(Some(&body));
    opts.method("POST");
    let url = "http://localhost:5000/register_post";
    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-type", "application/json")
        .expect("cannot add headers");
    let window = web_sys::window().expect("cannot get window");
    let res = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = res.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let resp_body: RegReturn = serde_wasm_bindgen::from_value(json)?;
    Ok(resp_body)
}
