use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::*;
use leptos_router::A;
#[component]
pub fn navbar() -> impl IntoView {
    view! {
        <nav class="bg-gray-950 p-4">
            <div class="container mx-auto flex justify-between items-center">
                <a href="/" class="text-white text-xl font-bold">
                    ws-explo
                </a>
                <ul class="flex space-x-4">
                    <li>
                        <A href="/send" class="text-gray-300 hover:text-white font-semibold">
                            Send
                        </A>
                    </li>
                    <li>
                        <A href="/login" class="text-gray-300 hover:text-white font-semibold">
                            Login
                        </A>
                    </li>
                    <li>
                        <A href="/account" class="text-gray-300 hover:text-white font-semibold">
                            Account
                        </A>
                    </li>
                    <li>
                        <A href="/register" class="text-gray-300 hover:text-white font-semibold">
                            Register
                        </A>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
