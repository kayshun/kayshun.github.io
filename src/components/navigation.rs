//! Holds the main navigation component for the front-end application
use leptos::*;
use thaw::{Grid, GridItem, Icon, Image, LayoutHeader, Space, SpaceJustify};

use crate::pages::Page;

/// The main navigation component is a horizontal bar with links to the main pages of the application
#[component]
pub fn MainNavigation() -> impl IntoView {
    view! {
        <LayoutHeader class="flex items-end justify-between p-5 border-b bg-gray-50">
            <Space>
            <Grid cols=3 x_gap=25 >

            <GridItem>
                <Space justify=SpaceJustify::Start>
                <div class="flex items-end">
                    // ------------------ Logo box ------------------
                    <a href=Page::Home.path() class="flex items-center space-x-2">
                        // <img class="responsive-img" src="/images/kayshun_flame_icon_orig.png" alt="Kayshun" />
                        <Image src="/images/kayshun_flame_icon_orig.png" width="60px" alt="Kayshun" />
                        <span class="font-oswald text-6xl"> kayshun</span>
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
            <div class="flex justify-end border border-gray-200 rounded-md">

                    <a href=Page::Home.path()
                       title="Go to home page"
                       class="text-gray-800 hover:text-orange-600 py-1 px-3">
                        <Icon icon=icondata::RiHome4BuildingsLine height="2.5em" width="3em"/>
                        <p class="font-oswald text-sm text-center">Home</p>
                    </a>

                    <a href=Page::OurProducts.path()
                title="Our products"
                    class="text-gray-800 hover:text-orange-600 py-1 px-3">
                        <Icon icon=icondata::RiRedPacketFinanceLine height="2.5em" width="3em"/>
                        <p class="font-oswald text-sm text-center">Product</p>
                    </a>

                    <a href=Page::AboutUs.path()
                       title="About us"
                       class="text-gray-800 hover:text-orange-600 py-1 px-3">
                        <Icon icon=icondata::IoPeopleCircleOutline height="2.5em" width="3em"/>
                        <p class="font-oswald text-sm text-center">About</p>
                    </a>

                    <a href=Page::ContactUs.path()
                       title="Contact us"
                       class="text-gray-800 hover:text-orange-600 py-1 px-3">
                        <Icon icon=icondata::RiMailBusinessLine height="2.5em" width="3em"/>
                        <p class="font-oswald text-sm text-center">Contact</p>
                    </a>

            </div>
            </Space>
            </GridItem>

            </Grid>
            </Space>
        </LayoutHeader>
    }
}
