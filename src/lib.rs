use chrono::{Datelike, Utc};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;
use thaw::Layout;

mod components;
mod pages;

use crate::pages::about_us::AboutUs;
use crate::pages::contact_us::ContactUs;
use crate::pages::faqs::Faqs;
use crate::pages::home::Home;
use crate::pages::not_found::FouroFour;
use crate::pages::our_products::Edu;
use crate::pages::Page;

use crate::components::navigation::MainNavigation;

/// The main App, the start-point if the webapp
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <body>
            <div class="flex flex-col h-screen overflow-hidden bg-orange-50">
            <MainNavigation/>
            <Router>
                <Layout class="lg:mx-96 mb-3 flex-1 overflow-y-scroll">
                <div class="bg-fixed bg-cover overflow-clip" style="background-image: url('./images/hs_classroom_orig.png')">
                    <Routes>
                        <Route path=Page::Home.path() view=Home/>
                        <Route path=Page::OurProducts.path() view=Edu/>
                        <Route path=Page::AboutUs.path() view=AboutUs/>
                        <Route path=Page::Faqs.path() view=Faqs/>
                        <Route path=Page::ContactUs.path() view=ContactUs/>

                        // ------------------ 404 page handling ------------------
                        <Route path=Page::NotFound.path() view=FouroFour />
                    </Routes>
                </div>
                </Layout>
            </Router>
            </div>
        </body>
        <footer class="p-4 text-center text-xs text-gray-400 font-thin bg-orange-50">
            {"© Copyright "}{Utc::now().year()}{" Kayshun Limited. All rights reserved."}
        </footer>
    }
}
