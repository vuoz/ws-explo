use crate::calls;
use leptos::component;
use leptos::create_node_ref;
use leptos::html::Input;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::*;

#[component]
pub fn login_view() -> impl IntoView {
    let name_ref: NodeRef<Input> = create_node_ref();
    let pass_ref: NodeRef<Input> = create_node_ref();

    let window = web_sys::window().expect("cannot get window");
    if let Ok(key) = window.local_storage().expect("").expect("").get("auth") {
        if key.is_some(){
            let navigate = leptos_router::use_navigate();
            navigate("/account", leptos_router::NavigateOptions::default())
        }
    }
    let on_sub = move |_| {
        let name = name_ref.get().expect("cannot get value").value();
        let pass = pass_ref.get().expect("cannot get value").value();
        if pass.is_empty()|| name.is_empty()  {
            return;
        }
        wasm_bindgen_futures::spawn_local(async move {
            let res = match calls::login::send_login(name, pass).await {
                Ok(res) => res,
                Err(_) => return,
            };

            let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            storage.set_item("auth", &res.token).unwrap();
            storage.set_item("cli_key", &res.key).unwrap();
            let navigate = leptos_router::use_navigate();
            navigate("/account", leptos_router::NavigateOptions::default())
        })
    };
    view! {
        <div class="w-full h-screen flex items-center content-center justify-center">
            <div class="w-[50%] outline outline-gray-900 h-[50%]  p-[10%]  space-y-4">
                <p class="text-white font-bold text-left">Login</p>
                <input
                    type="text"
                    class="  w-full p-2  rounded text-white font-semibold outline bg-transparent align-middle outline-gray-900"
                    placeholder="Name..."
                    name=""
                    id=""
                    node_ref=name_ref
                />
                <input
                    type="text"
                    class="  w-full p-2  rounded text-white font-semibold outline bg-transparent  align-middle outline-gray-900"
                    placeholder="Password..."
                    name=""
                    id=""
                    node_ref=pass_ref
                />
                <button
                    class="w-full p-2 rounded text-white hover:scale-105 font-semibold text-center align-middle  outline outline-black bg-transparent  flex items-center justify-center content-center "
                    on:click=on_sub
                >
                    Send
                </button>
            </div>
        </div>
    }
}
