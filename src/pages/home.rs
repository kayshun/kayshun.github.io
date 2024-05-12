use leptos::*;
use thaw::{Grid, GridItem, Layout, Space};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout class="p-5">
            <Space>
                <Grid class="p-2">
                    <GridItem>
                        <h1 class="font-oswald text-4xl text-gray-800 pb-4">"Reigniting education"</h1>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Our mission"</p>
                    </GridItem>
                </Grid>
            </Space>
        </Layout>
    }
}
