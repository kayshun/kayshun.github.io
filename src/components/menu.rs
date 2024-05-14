use leptos::*;
use thaw::{Button, ButtonVariant, Drawer, Grid, GridItem, Icon, Space};

use crate::Page;

/// Displays the menu for desktop
#[component]
pub fn WebMenu() -> impl IntoView {
    view! {
        <div class="flex justify-end">
            <a href=Page::Home.path()
               title="Go to home page"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiHome4BuildingsLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">Home</p>
            </a>

            <a href=Page::OurProducts.path()
                title="Our products"
                class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiRedPacketFinanceLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">Product</p>
            </a>

            <a href=Page::AboutUs.path()
               title="About us"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::IoPeopleCircleOutline height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">About</p>
            </a>

            <a href=Page::ContactUs.path()
               title="Contact us"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiMailBusinessLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">Contact</p>
            </a>
        </div>
    }
}

/// Displays the menu for mobile
#[component]
pub fn MobileMenu() -> impl IntoView {
    let show = create_rw_signal(false);

    view! {
        <Button
                variant=ButtonVariant::Link
                on_click=move |_| show.set(true)>
            <Icon
                class="text-gray-800 hover:text-orange-600"
                icon=icondata::RiMenuSystemFill height="3em" width="4em"/>
        </Button>

        <Drawer class="font-oswald text-xl" show width="160px" title="Menu">
            <Space vertical=true>
                <Grid>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::Home.path()
                           title="Go to home page"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::RiHome4BuildingsLine height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">Home</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::OurProducts.path()
                            title="Our products"
                            class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::RiRedPacketFinanceLine height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">Product</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::AboutUs.path()
                           title="About us"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::IoPeopleCircleOutline height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">About</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::RiMailBusinessLine height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">Contact</p>
                        </a>
                </Button>

                    </GridItem>

                </Grid>
            </Space>
        </Drawer>
    }
}
