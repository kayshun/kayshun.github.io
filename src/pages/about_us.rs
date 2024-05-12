use leptos::*;
use thaw::{Grid, GridItem, Image, Layout, Space};

/// The about us page
#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
        <Layout class="p-5">

            <Space>

                <Grid cols=2 class="p-2">

                    <GridItem column=2>
                        <h1 class="font-oswald text-4xl text-gray-800 pb-4">"About us"</h1>
                    </GridItem>

                    <GridItem class="p-2" column=2>
                        <h2 class="font-oswald text-3xl text-gray-800 pb-3">"Our founders"</h2>
                    </GridItem>

                    <GridItem class="p-2">
                        <h3 class="font-oswald text-2xl text-gray-700 pb-2">"Jean Clarke"</h3>
                        <Image width="300px" height="300px"/>
                    </GridItem>

                    <GridItem class="p-2">
                        <h3 class="font-oswald text-2xl text-gray-700 pb-2">"Mick Clarke"</h3>
                        <Image width="300px" height="300px"/>
                    </GridItem>

                </Grid>

            </Space>

        </Layout>
    }
}
