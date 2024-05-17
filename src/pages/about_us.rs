use leptos::*;
use leptos_meta::Title;
use std::collections::HashSet;
use thaw::{Collapse, CollapseItem, Grid, GridItem, Icon, Layout, Space};

use crate::components::{advisors, community, founders, tech_comp};
use crate::Page;
/// The about us page
#[component]
pub fn AboutUs() -> impl IntoView {
    let value = create_rw_signal(HashSet::from(["founders".to_string()]));

    view! {
        <Title text="Kayshun - About"/>

        <Layout class="w-full lg:max-w-6xl lg:mx-auto">
            <Space>
                <Grid>
                    <GridItem class="p-4 w-full lg:max-w-6xl lg:mx-auto">
                        <h1 class="font-oswald text-3xl text-gray-800">"About"</h1>
                    </GridItem>
                </Grid>
            </Space>
            <Space>
                <Collapse accordion=true value class="pb-5 w-full lg:max-w-6xl lg:mx-auto">
                    <CollapseItem class="font-oswald text-2xl text-gray-800" title="Our founders" key="founders">
                        <founders::About/>
                    </CollapseItem>
                    <CollapseItem class="font-oswald text-2xl text-gray-800" title="Our advisors" key="advisors">
                        <advisors::About/>
                    </CollapseItem>
                    <CollapseItem class="font-oswald text-2xl text-gray-800" title="Our community champions" key="community">
                        <community::About/>
                    </CollapseItem>
                    <CollapseItem class="font-oswald text-2xl text-gray-800" title="Technology & compliance" key="tech">
                        <tech_comp::About/>
                    </CollapseItem>
                </Collapse>
            </Space>
            <Space>
                <Grid class="pt-2">

                    <GridItem>
                        <h2 class="font-oswald text-2xl text-gray-800 pb-3">"Join our community"</h2>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-base text-gray-700">
                            <div class="mr-12">
                                "We want to be a community-driven initiative that improves education through digital products that are simple to use and solve real-world issues."
                            </div>
                        </p>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-base text-gray-700">
                            <div class="mr-12">
                        "Please help us improve our products and make them more usable and valuable to achieve our collective goals."
                            </div>
                        </p>
                    </GridItem>

                    <GridItem class="p-2">
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600 font-bold pb-2 py-1 px-3">
                            <Icon icon=icondata::RiMailSendBusinessLine height="2.5em" width="3em"/>
                            "Contact us"
                        </a>

                        <p class="text-sm text-gray-500 pt-6"><i>*We all need some edukayshun, init...</i></p>
                    </GridItem>

                </Grid>
            </Space>
        </Layout>

    }
}
