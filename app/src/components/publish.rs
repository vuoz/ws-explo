use crate::calls;
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
#[component]
pub fn publish_view() -> impl IntoView {
    let action = leptos::create_action(|inpt: &PubInput| send_to_server(inpt.clone()));
    let msg_ref: NodeRef<Input> = create_node_ref();
    let (canned, setCanned) = create_signal(false);
    let on_sub = move |cancel | {
        if cancel {
            setCanned(true);
        }
        let msg = msg_ref.get().expect("cannot get value").value();
        if msg.is_empty() {
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
        >
        <div class="w-full h-screen flex items-center content-center justify-center">
            <div class="w-[50%] outline outline-gray-900 h-[50%]  p-[10%]  space-y-4">
                <p class="text-white font-bold text-left">Send Msg</p>
                <input
                    type="text"
                    class="  w-full p-2  rounded text-white font-semibold outline bg-transparent align-middle outline-gray-900"
                    placeholder="Msg..."
                    name=""
                    id=""
                    node_ref=msg_ref
                />
                <button
                    class="w-full p-2 rounded text-white hover:scale-105 font-semibold text-center align-middle  outline outline-black bg-transparent  flex items-center justify-center content-center "
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
                                view! { <p class="text-red-900 ">{v}</p> }.into_view()
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
