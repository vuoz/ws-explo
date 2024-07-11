use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use web_sys::{Request, RequestInit};
use crate::calls::login::FetchError;

use super::login::AuthedUser;





pub async fn check_auth(token:String) -> Result<(), FetchError> {
    let body_struct = AuthedUser{
        token,
        key:"".to_string()
    };
    let body_str = serde_json::to_string(&body_struct)?;
    let body = serde_wasm_bindgen::to_value(&body_str)?;
    let mut opts = RequestInit::new();
    opts.body(Some(&body));
    opts.method("POST");
    let url = "http://localhost:5000/check_auth";
    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Content-type", "application/json").expect("cannot add headers");
    let window = web_sys::window().expect("cannot get window");
    let res = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = match res.dyn_into() {
        Ok(resp) => resp,
        Err(_) => return Err(FetchError::DynError),
    };
    if resp.status() == 200 {
        Ok(())
    }else if resp.status() == 403{
         Err(FetchError::TokenError)
    }else{

        let text_resp = JsFuture::from(resp.text()?).await?;
        leptos::logging::log!("Response from check_auth: {:?}", text_resp);
         Err(FetchError::StatusError(resp.status()))
    }
}
