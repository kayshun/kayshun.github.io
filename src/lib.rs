use leptos::*;
use leptos_router::*;

mod components;
mod pages;

use crate::pages::home::Home;
use crate::pages::not_found::FouroFour;
use crate::pages::Page;

use crate::components::navigation::MainNavigation;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container rounded-lg mx-auto p-2 pt-2 mt-5 border border-gray-200 bg-gray-50 ">
            <Router>
                <MainNavigation/>
                <Routes>
                    <Route path=Page::Home.path() view=Home/>
                    // ------------------ 404 page handling ------------------
                    <Route path=Page::NotFound.path() view=FouroFour />
                </Routes>
            </Router>
        </div>
    }
}
