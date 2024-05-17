use leptos::*;
use thaw::{Button, ButtonVariant, Grid, GridItem, Image, Modal};

use crate::components::about::BioPanel;
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
                "While we can leverage our own background, we need to learn more and be challenged on what the best path forward to take. We are lucky to be working with super experienced advisors to help guide our journey."
                </div>
            </p>
        </GridItem>

        <GridItem class="ml-8">
            <div class="container max-w-fit columns-2">
                <div class="p-2">
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Arabella Northey"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_one.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            // src="/images/jean_headshot.jpg"
                            width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_one>
                        <BioPanel
                            name="Arabella Northey".to_string()
                            image_path="".to_string()
                            li_url="https://www.linkedin.com/in/arabella-northey2208/".to_string()
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
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Jo Kennedy"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_two.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            // src="/images/mick_headshot_sepia.jpg"
                            width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_two>
                        <BioPanel
                            name="Jo Kennedy".to_string()
                            image_path="".to_string()
                            // li_url="https://www.linkedin.com/in/mickclarke138/".to_string()
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
