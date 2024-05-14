use chrono::{Datelike, Utc};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;
use thaw::Layout;

mod components;
mod pages;

use crate::pages::about_us::AboutUs;
use crate::pages::contact_us::ContactUs;
use crate::pages::home::Home;
use crate::pages::not_found::FouroFour;
use crate::pages::our_products::OurCommunity;
use crate::pages::Page;

use crate::components::navigation::MainNavigation;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <body class="mx-auto min-h-screen lg:flex lg:flex-col bg-orange-50">
            <Router>
                <Layout class="w-14/15 mx-auto p-2 mt-5 border border-gray-200 rounded-lg bg-gray-50">
                    <MainNavigation/>
                    <Routes>
                        <Route path=Page::Home.path() view=Home/>
                        <Route path=Page::OurProducts.path() view=OurCommunity/>
                        <Route path=Page::AboutUs.path() view=AboutUs/>
                        <Route path=Page::ContactUs.path() view=ContactUs/>

                        // ------------------ 404 page handling ------------------
                        <Route path=Page::NotFound.path() view=FouroFour />
                    </Routes>
                </Layout>
            </Router>
        </body>
        <footer class="p-4 text-center text-xs text-gray-400 font-thin bg-orange-50">
            {"© Copyright "}{Utc::now().year()}{" Kayshun Limited. All rights reserved."}
        </footer>
    }
}
