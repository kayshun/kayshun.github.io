use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Layout, Space};

use crate::pages::Page;

/// 404 Not Found Page
#[component]
pub fn FouroFour() -> impl IntoView {
    view! {
        <Title text="Kayshun - Not Found"/>
        <Layout class="w-full lg:max-w-6xl lg:mx-auto">
            <Space>
                <Grid class="w-full text-gray-700">
                    <GridItem class="w-full p-6">
                        <h1 class="font-oswald text-3xl">"Page not found!"</h1>
                    </GridItem>
                    <GridItem class="w-full p-6">
                        <p class="mb-2 text-xl font-light tracking-tight text-gray-600">
                            {"Oh dear, looks like you have got your self lost there..."}
                        </p>
                        <p class="mb-2 pt-4 text-lg font-light tracking-tight text-gray-600">
                            {"Please go back to the"}<a href=Page::Home.path() class="underline text-gray-800 hover:text-orange-600 font-bold py-0.5 px-1">Home Page.</a>
                        </p>
                    </GridItem>
                </Grid>
            </Space>
        </Layout>
    }
}
