use leptos::*;
use thaw::{Button, ButtonVariant, Grid, GridItem, Image, Modal};

use crate::components::about::BioPanel;
/// The about us page
#[component]
pub fn About() -> impl IntoView {
    let show_bella = create_rw_signal(false);
    let show_jo = create_rw_signal(false);

    view! {

        <Grid cols=2 class="font text-base text-gray-700">

            <GridItem column=2 class="pb-2 bg-orange-400">
                <p class="font-sans text-base text-white max-w-prose">"We are lucky to be working with super experienced advisors to help guide our journey."</p>
            </GridItem>

            <GridItem class="pl-8">
                <h3 class="font-oswald text-xl text-gray-700 pb-2">"Arabella Northey"</h3>
                <div class="pt-2">

                    <Button
                        variant=ButtonVariant::Link
                        on_click=move |_| { show_bella.set(true) }>

                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                             width="100px" height="100px"/>

                    </Button>
                    <Modal
                        title=""
                        width="425px"
                        show=show_bella>
                        <BioPanel
                            name="Arabella Northey".to_string()
                            image_path="".to_string()
                            li_url="https://www.linkedin.com/in/arabella-northey2208/".to_string()
                        >
                            <p class="pt-2 pb-2">
                            "Bella's bio. Paragraph one."
                            </p>
                            <p class="pb-2">
                            "Bella's bio. Paragraph two."
                            </p>

                        </BioPanel>
                    </Modal>

                </div>

            </GridItem>

            <GridItem class="">
                <h3 class="font-oswald text-xl text-gray-700 pb-2">"Jo Kennedy"</h3>
                <div class="pt-2">

                    <Button variant=ButtonVariant::Link on_click=move |_| { show_jo.set(true) }>

                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            width="100px" height="100px"/>

                    </Button>
                    <Modal
                        title=""
                        width="425px"
                        show=show_jo>
                        <BioPanel
                            name="Jo Kennedy".to_string()
                            image_path="".to_string()
                        >
                            <p class="pt-2 pb-2">
                               "Jo'a bio. Paragraph one."
                            </p>
                            <p class="pb-2">
                                "Jo'a bio. Paragraph two."
                            </p>
                        </BioPanel>
                    </Modal>

                </div>
            </GridItem>

            <GridItem class="p-4" column=2>
                <p class="font-oswald p-2 text-sm italic">Click their image to see their bio</p>
            </GridItem>

        </Grid>
    }
}
