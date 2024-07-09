use crate::calls;
use crate::calls::client_active::ClientActive;
use leptos_router::use_navigate;
use leptos::component;
use leptos::create_node_ref;
use leptos::html::Input;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::*;

async fn send_to_server(inpt: PubInput) -> String {
    match calls::publish::send_add(inpt.msg.to_string(), inpt.key.to_string(), inpt.cancel).await {
        Ok(_) => String::from(""),
        Err(e) => e.to_string(),
    }
}
#[derive(Clone)]
struct PubInput {
    key: String,
    msg: String,
    cancel: bool,
}
impl PubInput {
    fn new(key: String, msg: String, cancel: bool) -> Self {
        PubInput { key, msg, cancel }
    }
}
async fn get_client_active_wrapper() -> Option<ClientActive>{
        match web_sys::window()
            .expect("cannot get window")
            .local_storage()
            .expect("cannot get storage")
            .expect("cannot get storage")
            .get("auth")
            .expect("cannot get")
        {
            Some(key) =>{
                match calls::client_active::get_client_active(key).await{
                Ok(v) => Some(v),
                Err(calls::publish::FetchError::Forbidden)=> {
                    web_sys::window().expect("").local_storage().expect("").expect("").remove_item("auth").expect("error removing");
                    let navigate =  use_navigate(); navigate("/login",leptos_router::NavigateOptions::default() );None
                },
                _ => None

            }

        },
            None => { let navigate =  use_navigate(); navigate("/login",leptos_router::NavigateOptions::default() );None}
        }

}

#[component]
pub fn publish_view() -> impl IntoView {
    let action = leptos::create_action(|inpt: &PubInput| send_to_server(inpt.clone()));
    let msg_ref: NodeRef<Input> = create_node_ref();
    let (canned, setCanned) = create_signal(false);
        
        
    let resource = create_resource(move || (), |_| async move{
        get_client_active_wrapper().await

    });
   
    let on_sub = move |cancel | {
        if cancel {
            setCanned(true);
        }
        let msg = msg_ref.get().expect("cannot get value").value();
        if msg.is_empty() && !cancel { 
            return;
        }
        let key = match web_sys::window()
            .expect("cannot get window")
            .local_storage()
            .expect("cannot get storage")
            .expect("cannot get storage")
            .get("auth")
            .expect("cannot get")
        {
            Some(key) => key,
            None => return,
        };
        if key.is_empty() {
            return;
        }

        action.dispatch(PubInput::new(key, msg, cancel));
    };
    view! {
        <div class="w-full h-screen flex items-center content-center justify-center">
            <div class="w-[50%] outline outline-gray-900 h-[50%]  p-[10%]  space-y-4">
                <p class="text-white font-bold text-left">Send Msg</p>
                <p class="  font-light text-center text-gray-600">
                    **
                    Your cli connection will be canceled after 5 mins to avoid overcrowding the server with tcp connections
                    **

                </p>
                <input
                    type="text"
                    class="  w-full p-2  rounded text-white font-semibold outline bg-transparent align-middle outline-gray-900"
                    placeholder="Msg..."
                    name=""
                    id=""
                    node_ref=msg_ref
                />
                <button
                    class="w-full p-2 rounded text-white hover:scale-105 font-semibold text-center align-middle  outline outline-zinc-400 bg-transparent  flex items-center justify-center content-center "
                    on:click=move |_| { on_sub(false) }
                >
                    Send to client
                </button>
                <button
                    class="w-full p-2 rounded text-red outline outline-red-600   font-semibold  text-red-600 align-middle bg-transparent hover:scale-105"
                    on:click=move |_| { on_sub(true) }
                >
                    Cancel client connection!
                </button>
                <Transition fallback=move || {
                    view! { <div></div> }
                }>
                    {move || match resource.get() {
                        None => {
                            view! { <div class="text-whitie">Error loading client status</div> }
                                .into_view()
                        }
                        Some(v) => {
                            {
                                if let Some(v) = v {
                                    match v.active {
                                        true => {
                                            view! { <div class="text-green-300">Client is active</div> }
                                        }
                                        false => {
                                            view! {
                                                <div class="text-red-400">Client is not active</div>
                                            }
                                        }
                                    }
                                        .into_view()
                                } else {
                                    view! {}.into_view()
                                }
                            }
                                .into_view()
                        }
                    }}

                </Transition>
                {move || {
                    let data = action.value();
                    match data() {
                        None => {
                            {
                                view! {}
                            }
                                .into_view()
                        }
                        Some(v) => {
                            if v.is_empty() {
                                view! {}.into_view()
                            } else {
                                view! { <p class="text-red-400 ">{v}</p> }.into_view()
                            }
                        }
                    }
                }}

                {move || {
                    match canned() {
                        false => {
                            {
                                view! {}
                            }
                                .into_view()
                        }
                        true => {
                            {
                                view! { <p class="text-red-900 text-left">Client was canceled</p> }
                            }
                                .into_view()
                        }
                    }
                }}

            </div>
        </div>
    }
}
