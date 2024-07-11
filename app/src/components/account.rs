use leptos::component;
use leptos_router::use_navigate;
use leptos::view;
use leptos::IntoView;
use leptos::*;

use crate::calls;

async fn get_check_auth_wrapper()-> Option<()>{
    match web_sys::window().expect("").local_storage().expect("").expect("").get_item("auth").expect("cannot get from local_storage"){
       Some(token) =>{

            match calls::check_auth::check_auth(token).await{
                Ok(_)=> Some(()),
                Err(calls::login::FetchError::TokenError) => {
                    web_sys::window().expect("").local_storage().expect("").expect("").remove_item("auth").expect("error removing");
                    let navigate =  use_navigate(); navigate("/login",leptos_router::NavigateOptions::default() );
                    None
                },
                _ => None
            }

       }
       None=>{
          let navigate =  use_navigate();
          navigate("/login",leptos_router::NavigateOptions::default() );
          None
       }

    }



}

#[component]
pub fn account() -> impl IntoView {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
   let resource = create_resource(move || (), |_| async move{
        get_check_auth_wrapper().await
    });
    let key = match storage.get("cli_key") {
        Ok(key) => match key {
            None => {
                let navigate = leptos_router::use_navigate();
                navigate("/login", leptos_router::NavigateOptions::default());
                return view! {}.into_view();
            }
            Some(key) => key,
        },
        Err(_) => {
            let navigate = leptos_router::use_navigate();
            navigate("/login", leptos_router::NavigateOptions::default());
            return view! {}.into_view();
        }
    };
    let (key_state, _) = create_signal(key.clone());
    let copy_key = move |text: &str| {
        // web_sys clipboard api is unstable though. Therefore js_sys::eval is used
        let _ = js_sys::eval(
            &("let text = ".to_owned()
                + "'"
                + text
                + "';"
                + "navigator.clipboard.writeText(text);"),
        );
    };
    let on_click = move |_| {
        let text = key_state();
        copy_key(text.as_str());
        let navigate = leptos_router::use_navigate();
        navigate("/send", leptos_router::NavigateOptions::default());
    };

    view! {
        <div class="w-full h-screen flex items-center content-center justify-center">
            <div class="w-[50%] outline outline-gray-900 h-[50%]  p-[10%]  space-y-1">
                <div class="text-left font-bold text-white">Your Key!</div>
                <div class="outline-gray-900 outline rounded p-2 text-center text-white text-align">
                    {key}
                </div>
                <div class="w-full mt-4 pt-2 space-y-1 flex items-center content-center justify-center flex-col ">
                    <button
                        on:click=on_click
                        class="outline w-full  outline-green-400 hover:scale-105 rounded  p-2 text-center text-green-400 align-middle"
                        type=""
                    >
                        Go to Send!
                    </button>
                    <p class="text-left text-green-400 text-sm">Your key will be copied.</p>
                    <Transition fallback=move || {
                        view! {}.into_view()
                    }>
                        {move || match resource.get() {
                            None => view! {}.into_view(),
                            Some(_) => view! {}.into_view(),
                        }}

                    </Transition>

                </div>

            </div>
        </div>
    }
    .into_view()
}
