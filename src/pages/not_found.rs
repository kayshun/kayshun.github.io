use leptos::*;
use leptos_meta::Title;

use crate::pages::Page;

/// 404 Not Found Page
#[component]
pub fn FouroFour() -> impl IntoView {
    view! {
        <Title text="Kayshun - Not Found"/>
        <div class="container rounded-lg mx-auto p-2 pt-2 bg-gray-50">
        <div class="flex flex-col pt-2 ">
            <h1 class="mb-4 text-2xl font-bold text-orange-600">{"Page not found!"}</h1>
            <p class="mb-2 text-xl font-light tracking-tight text-gray-600">
                {"Oh dear, looks like you have got your self lost there..."}
            </p>
            <p class="mb-2 pt-4 text-lg font-light tracking-tight text-gray-600">
                {"Please go back to the"}<a href=Page::Home.path() class="underline text-gray-800 hover:text-orange-600 font-bold py-0.5 px-1">Home Page.</a>
            </p>
        </div>
        </div>
    }
}
