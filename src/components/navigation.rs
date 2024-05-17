//! Holds the main navigation component for the front-end application
use leptos::*;
use thaw::Image;

use crate::components::menu::{MobileMenu, WebMenu};
use crate::pages::Page;

/// The main navigation component is a horizontal bar with links to the main pages of the application
#[component]
pub fn MainNavigation() -> impl IntoView {
    view! {
        <header class="w-full px-4 lg:max-w-6xl lg:mx-auto pt-3 pb-3 bg-gray-50">
            <div class="flex items-center justify-between">
                // Logo box
                <div class="flex space-x-2">
                    <a href=Page::Home.path() class="flex space-x-2">
                        <Image src="/images/kayshun_flame_icon_orig.png" width="40px" alt="Kayshun" />
                        <span class="font-oswald text-4xl text-orange-600"> Kayshun</span>
                    </a>
                </div>
                // Navigation links
                <div class="flex items-center">
                    <div class="hidden lg:flex">
                        <WebMenu/>
                    </div>
                    <div class="flex lg:hidden">
                        <MobileMenu/>
                    </div>
                </div>
            </div>
        </header>
    }
}
