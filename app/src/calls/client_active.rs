use crate::calls::publish::FetchError;
use crate::calls::publish::ErrorResp;



use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActive{
    pub active:bool
}




pub async fn get_client_active( token: String) -> Result<ClientActive, FetchError> {
    let mut opts = RequestInit::new();
    opts.body(None);
    opts.method("GET");
    let url = "http://localhost:5000/client_active";
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
    let resp_ = JsFuture::from(resp.json()?).await?;
    let json_client_active: ClientActive = serde_wasm_bindgen::from_value(resp_ )?;
    Ok(json_client_active)
}
