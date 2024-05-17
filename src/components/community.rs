use leptos::*;
use thaw::{Button, ButtonVariant, Grid, GridItem, Icon, Image, Modal, Space};

use crate::components::about::BioPanel;
use crate::Page;
/// The about us page
#[component]
pub fn About() -> impl IntoView {
    let show_one = create_rw_signal(false);
    let show_two = create_rw_signal(false);

    view! {

    <Grid class="w-full font text-base text-gray-700">
        <GridItem class="w-full p-6 bg-gradient-to-l from-orange-500 from-3% via-orange-400 via-80% to to-orange-300">
            <p class="font-sans font-light text-base text-white">
                <div class="mr-12">
                "We would not be anywhere without the people who are both advising us and championing us into the teacher community. These are more than just users or testers of our service, their input has made our product what it is."
                </div>
            </p>
        </GridItem>

        <GridItem class="ml-8">
            <div class="container max-w-fit columns-2">
                <div class="p-2">
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Person one"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_one.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            // src=""
                            width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_one>
                        <BioPanel
                            name="Person one".to_string()
                            image_path="".to_string()
                            li_url="".to_string()
                        >
                            <p class="pt-2 pb-2">
                                "Bio placeholder"
                            </p>
                            <p class="pb-2">
                                ""
                            </p>
                        </BioPanel>
                    </Modal>
                </div>
                <div class="p-2">
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Person two"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_two.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            // src=""
                            width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_two>
                        <BioPanel
                            name="Person two".to_string()
                            image_path="".to_string()
                            // li_url="".to_string()
                        >
                            <p class="pt-2 pb-2">
                                "Bio placeholder"
                            </p>
                            <p class="pb-2">
                                ""
                            </p>
                        </BioPanel>
                    </Modal>
                </div>
            </div>
        </GridItem>

        <GridItem class="p-4 w-full">
            <p class="font-oswald p-2 text-sm italic">Click their image to see their bio</p>
        </GridItem>
    </Grid>
    }
}

/// The about us page
#[component]
pub fn Community() -> impl IntoView {
    view! {
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
            </GridItem>

        </Grid>
    </Space>
    }
}
