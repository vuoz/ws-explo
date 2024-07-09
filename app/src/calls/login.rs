use serde::*;
use serde_wasm_bindgen::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use web_sys::{Request, RequestInit};


#[derive(Serialize, Deserialize, Clone)]
pub struct UserLoginReq{
    pub name :String,
    pub pass : String,
}



#[derive(Serialize, Deserialize, Clone)]
pub struct AuthedUser {
    pub token: String,
    pub key: String,
    // will add another layer of authentication in the future
}
pub enum FetchError {
    JsValue(JsValue),
    Error(serde_wasm_bindgen::Error),
    SerdeError(serde_json::Error),
    DynError,
    TokenError,
    StatusError(u16),
}
//this is just for debugging
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::DynError => {
                writeln!(f, "dyn into error")
            }
            FetchError::Error(e) => {
                writeln!(f, "SerdeWasmBindgenError {:?}", e)
            }
            FetchError::JsValue(e) => {
                writeln!(f, "JsBindingError {:?}", e)
            }
            FetchError::SerdeError(e) => {
                writeln!(f, "SerdeError {:?}", e)
            }
            FetchError::StatusError(code) => {
                writeln!(f, "status code {}", code)
            }
            FetchError::TokenError => {
                writeln!(f, "auth token error")
            }
        }
    }
}
impl From<Error> for FetchError {
    fn from(value: Error) -> Self {
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
impl From<jsonwebtoken::errors::Error> for FetchError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        FetchError::TokenError
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

pub async fn send_login(name: String, pass: String) -> Result<AuthedUser, FetchError> {
    let new_user = UserLoginReq {
        name,
        pass,
    };
    let mut opts = RequestInit::new();
    let body_str = serde_json::to_string(&new_user)?;
    let body = serde_wasm_bindgen::to_value(&body_str)?;
    opts.body(Some(&body));
    opts.method("POST");
    let url = "http://localhost:5000/login_post";
    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-type", "application/json")
        .expect("cannot add headers");
    let window = web_sys::window().expect("cannot get window");
    let res = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = match res.dyn_into() {
        Ok(resp) => resp,
        Err(_) => return Err(FetchError::DynError),
    };
    if resp.status() != 200 {
        return Err(FetchError::StatusError(resp.status()));
    }
    let json = JsFuture::from(resp.json()?).await?;
    let resp_body: AuthedUser = serde_wasm_bindgen::from_value(json)?;
    Ok(resp_body)
}
