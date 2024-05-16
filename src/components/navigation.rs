//! Holds the main navigation component for the front-end application
use leptos::*;
use thaw::{Grid, GridItem, Image, Space, SpaceJustify};

use crate::components::menu::{MobileMenu, WebMenu};
use crate::pages::Page;

/// The main navigation component is a horizontal bar with links to the main pages of the application
#[component]
pub fn MainNavigation() -> impl IntoView {
    view! {
        <header class="lg:mx-96 flex items-end justify-between p-3 bg-gray-50">
            <Space>
            <Grid cols=3 x_gap=50 >

            <GridItem>
                <Space justify=SpaceJustify::Start>
                <div class="flex items-end">
                    // ------------------ Logo box ------------------
                    <a href=Page::Home.path() class="flex items-center space-x-2">
                        <Image src="/images/kayshun_flame_icon_orig.png" width="40px" alt="Kayshun" />
                        <span class="font-oswald text-4xl text-orange-600"> Kayshun</span>
                    </a>
                </div>
                </Space>
            </GridItem>

            <GridItem>
                <Space justify=SpaceJustify::Center>
                    <span class="pl-3"/>
                </Space>
            </GridItem>

            <GridItem>
            <Space justify=SpaceJustify::End>
            // ------------------ Navigation links ------------------
                <div class="invisible lg:visible flex space-x-1">
                    <WebMenu/>
                </div>
                <div class="visible lg:invisible flex space-x-1">
                    <MobileMenu/>
                </div>
            </Space>
            </GridItem>

            </Grid>
            </Space>
        </header>
    }
}
