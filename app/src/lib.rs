pub mod calls;
pub mod components;
use crate::components::account::Account;
use crate::components::index::Index;
use crate::components::login::LoginView;
use crate::components::navbar::Navbar;
use crate::components::publish::PublishView;
use crate::components::register::RegisterView;
use leptos::component;
use leptos::view;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_meta::Body;
use leptos_meta::Title;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

#[component]
pub fn App() -> impl leptos::IntoView {
    provide_meta_context();
    view! {
        <Body class="bg-gray-950"/>
        <Title text="Welcome"/>
        <Router>
            <Navbar/>
            <main>

                <Routes>
                    <Route path="/" view=Index/>
                    <Route path="/login" view=LoginView/>
                    <Route path="/register" view=RegisterView/>
                    <Route path="/send" view=PublishView/>
                    <Route path="/account" view=Account/>
                </Routes>
            </main>
        </Router>
    }
}
