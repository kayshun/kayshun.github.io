use leptos::*;
use thaw::{Grid, GridItem, Icon, Image, Layout, Space};

use crate::pages::Page;
/// Default Home Page
#[component]
pub fn OurCommunity() -> impl IntoView {
    view! {
        <Layout class="p-5">
            <Space>
                <Grid class="p-2 max-w-prose">

                    <GridItem>
                        <h1 class="font-oswald text-4xl text-gray-800 pb-4">"Our products"</h1>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Our Edu app makes it simple to grade assignments and give quality and consistent feedback to your students."</p>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Watch our demo."</p>
                        <Image width="300px" height="300px"/>
                    </GridItem>

                </Grid>
            </Space>

            <Space>
                <Grid class="p-2 max-w-prose">

                    <GridItem>
                        <h2 class="font-oswald text-3xl text-gray-800 pb-3">"Join our community"</h2>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"We aim to be a community-driven initiative, with a goal of collectively improving education with digital products that are simple to use and solve real-world issues. To achieve our goals, we need you to help us improve  our products and make them even more usable and useful."</p>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Contact us to find out how you can  be a part of our community and get on our early users team."</p>
                    </GridItem>
                    <GridItem class="p-2">
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600 font-bold py-1 px-3">
                            <Icon icon=icondata::RiMailSendBusinessLine height="2.5em" width="3em"/>
                            "Contact us"
                        </a>
                    </GridItem>
                </Grid>
            </Space>
        </Layout>
    }
}
