use core::fmt;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

pub enum FetchError {
    JsValue(JsValue),
    Error(serde_wasm_bindgen::Error),
    SerdeError(serde_json::Error),
    DynError,
    StatusError,
    WindowError,
    Forbidden,
    ServerReason(String),
}
impl From<serde_wasm_bindgen::Error> for FetchError {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        FetchError::Error(value)
    }
}
impl From<serde_json::Error> for FetchError {
    fn from(value: serde_json::Error) -> Self {
        FetchError::SerdeError(value)
    }
}
impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError::JsValue(value)
    }
}
impl std::fmt::Display for FetchError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            FetchError::JsValue(_) => write!(f,"Please try again!"),
            FetchError::Error(_) => write!(f,"Error  unpacking data!"),
            FetchError::SerdeError(_) => write!(f,"Error unpacking data!"),
            FetchError::DynError => write!(f, "Error unpacking data!"),
            FetchError::StatusError => write!(f,"Please try again later!"),
            FetchError::WindowError => write!(f,"Please try again! "),
            FetchError::ServerReason(v) => write!(f,"{}",v),
            FetchError::Forbidden => write!(f,"Forbidden"),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgForFetch {
    msg: String,
    close: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResp {
    pub error: String,
}

pub async fn send_add(msg: String, token: String, cancel: bool) -> Result<(), FetchError> {
    let new_msg = MsgForFetch { msg, close: cancel };
    let mut opts = RequestInit::new();
    let body_str = serde_json::to_string(&new_msg)?;
    let body = serde_wasm_bindgen::to_value(&body_str)?;
    opts.body(Some(&body));
    opts.method("POST");
    let url = "http://localhost:5000/publish";
    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-type", "application/json")
        .expect("cannot add headers");
    request
        .headers()
        .set("x-auth", &token)
        .expect("cannote add headers");
    let window = web_sys::window().ok_or(FetchError::WindowError)?;
    let res = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = match res.dyn_into() {
        Ok(resp) => resp,
        Err(_) => return Err(FetchError::DynError),
    };
    if resp.status() != 200 {
        let resp_reason = JsFuture::from(resp.json()?).await?;
        let json_reason: ErrorResp = serde_wasm_bindgen::from_value(resp_reason)?;
        return Err(FetchError::ServerReason(json_reason.error));
    }
    Ok(())
}
