use leptos::*;
use leptos_meta::Title;
use thaw::{Collapse, CollapseItem, Grid, GridItem, Icon, Layout, Space};

use crate::Page;

/// Holds the FAQ information for the site.
#[component]
pub fn Faqs() -> impl IntoView {
    view! {
        <Title text="Kayshun - FAQs"/>
        <Layout class="p-5">

            <Space>
                <Grid class="p-2">
                    <GridItem>
                        <h1 class="font-oswald text-3xl pb-4">"FAQs"</h1>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-base pb-2">"We know you'll have lots of questions on our service. The following are the most commonly asked."</p>

                        <p class="text-base">"Didn't find what you are looking for? Ask away"
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600 pb-2 py-1 px-3">
                            <Icon icon=icondata::RiMailSendBusinessLine height="2.5em" width="3em"/>
                            " on our Contact us page!"
                        </a></p>

                    </GridItem>
                </Grid>
            </Space>

            <Space>

                <Collapse accordion=true class="pb-5">

                    <CollapseItem class="border-none font-oswald text-2xl text-gray-800" title="Information security and privacy" key="info_sec">
                        <p class="font-sans text-base text-gray-700">"Info sec stuff"</p>
                    </CollapseItem>

                    <CollapseItem class="border-none font-oswald text-2xl text-gray-800" title="Our use of AI" key="ai">
                        "AI stuff"
                    </CollapseItem>

                    <CollapseItem class="border-hidden font-oswald text-2xl text-gray-800" title="Countries we support" key="global">
                        "Roll out"
                    </CollapseItem>

                    <CollapseItem class="border-hidden font-oswald text-2xl text-gray-800" title="Technology" key="tech">
                        "A technology overview"
                    </CollapseItem>

                </Collapse>

            </Space>
        </Layout>
    }
}
