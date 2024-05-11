//! Holds the main navigation component for the front-end application
use leptos::*;
use thaw::Icon;

use crate::pages::Page;

/// The main navigation component is a horizontal bar with links to the main pages of the application
#[component]
pub fn MainNavigation() -> impl IntoView {
    view! {
        <nav class="relative flex items-end justify-between p-3 pt-3 border-b bg-gray-50">
            <div class="flex items-end">
                // ------------------ Logo box ------------------
                <a href=Page::Home.path() class="flex items-center space-x-2">
                    <img class="w-full sm:w-3/4 md:w-2/5 lg:w-1/4 xl:w-1/5" src="/images/kayshun_logo_txt.png" alt="Kayshun" />
                </a>

            </div>
            // ------------------ Navigation links ------------------
            <div class="flex space-x-1 mb-2 border border-gray-200 rounded-md">

                    <a href=Page::Home.path()
                       title="Go to home page"
                       class="text-gray-800 hover:text-orange-600 font-bold py-1 px-3">
                        <Icon icon=icondata::RiHome4BuildingsLine height="2.5em" width="3em"/>
                    </a>

            </div>
        </nav>
    }
}
