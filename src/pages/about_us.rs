use leptos::*;
use leptos_meta::Title;
use std::collections::HashSet;
use thaw::{Collapse, CollapseItem, Grid, GridItem, Layout, Space};

use crate::components::{advisors, community, founders, tech_comp};
/// The about us page
#[component]
pub fn AboutUs() -> impl IntoView {
    let value = create_rw_signal(HashSet::from(["founders".to_string()]));

    view! {
        <Title text="Kayshun - About"/>

        <Layout class="p-5">

            <Space>

                <Grid class="p-2">

                    <GridItem class="pb-2">
                        <h1 class="font-oswald text-3xl text-gray-800">"About"</h1>
                    </GridItem>

                </Grid>

            </Space>

            <Space>

                <Collapse accordion=true value>

                    <CollapseItem class="font-oswald text-2xl text-gray-800 p-2" title="Our founders" key="founders">
                        <founders::About/>
                    </CollapseItem>

                    <CollapseItem class="font-oswald text-2xl text-gray-800 p-2" title="Our advisors" key="advisors">
                        <advisors::About/>
                    </CollapseItem>

                    <CollapseItem class="font-oswald text-2xl text-gray-800 p-2" title="Our community champions" key="community">
                        <community::About/>
                    </CollapseItem>

                    <CollapseItem class="font-oswald text-2xl text-gray-800 p-2" title="Technology & compliance" key="tech">
                        <tech_comp::About/>
                    </CollapseItem>

                </Collapse>

            </Space>

        </Layout>

    }
}
