use leptos::*;
use leptos_router::A;
#[component]
pub fn index() -> impl IntoView {
    view! {
        <section class="w-full h-screen flex items-center content-center justify-center">
            <div class="w-[50%] outline outline-gray-900 h-[50%]  p-[10%]  space-y-4">
                <p class="text-white font-bold text-center text-2xl">Welcome</p>
                <A
                    href="/login"
                    class="w-full p-2 rounded text-white hover:scale-105 font-semibold text-center align-middle outline outline-zinc-400 bg-transparent  flex items-center justify-center content-center "
                >
                    Login
                </A>
                <A
                    href="/register"
                    class="w-full p-2 rounded text-white hover:scale-105 font-semibold text-center align-middle outline outline-zinc-400 bg-transparent  flex items-center justify-center content-center "
                >
                    Register
                </A>
            </div>
        </section>
    }
}
