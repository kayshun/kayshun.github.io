use chrono::{Datelike, Utc};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container">
            <div class="text-container p-4">
                <p>reigniting education, soon...</p>
            </div>
        </div>
        {
            // Footer
            view! {
                <footer class="p-4 text-center text-xs text-gray-300 font-thin">
                    {"© Copyright "}{Utc::now().year()}{" Kayshun Limited. All rights reserved."}
                </footer>
            }
        }
    }
}
