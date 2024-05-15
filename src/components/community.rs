use leptos::*;
use thaw::{Button, ButtonVariant, Grid, GridItem, Image, Modal};

use crate::components::about::BioPanel;
/// The about us page
#[component]
pub fn About() -> impl IntoView {
    let show_doug = create_rw_signal(false);
    let show_cheryl = create_rw_signal(false);

    view! {

        <Grid cols=2 class="font text-base text-gray-700 p-2">

            <GridItem column=2 class="max-w-prose pb-2">
                <p class="font-sans text-base text-gray-700">"We would not be anywhere without the people who are both advising us and championing us into the teacher community."</p>
            </GridItem>

            <GridItem class="pl-8">
                <h3 class="font-oswald text-xl text-gray-700 pb-2">"Doug"</h3>
                <div class="pt-2">

                    <Button
                        variant=ButtonVariant::Link
                        on_click=move |_| { show_doug.set(true) }>

                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                             width="100px" height="100px"/>

                    </Button>
                    <Modal
                        title=""
                        width="425px"
                        show=show_doug>
                        <BioPanel
                            name="Doug".to_string()
                            image_path="".to_string()
                        >
                            <p class="pt-2 pb-2">
                            "Doug's bio. Paragraph one."
                            </p>
                            <p class="pb-2">
                            "Doug's bio. Paragraph two."
                            </p>

                        </BioPanel>
                    </Modal>

                </div>

            </GridItem>

            <GridItem class="">
                <h3 class="font-oswald text-xl text-gray-700 pb-2">"Cheryl"</h3>
                <div class="pt-2">

                    <Button variant=ButtonVariant::Link on_click=move |_| { show_cheryl.set(true) }>

                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            width="100px" height="100px"/>

                    </Button>
                    <Modal
                        title=""
                        width="425px"
                        show=show_cheryl>
                        <BioPanel
                            name="Cheryl".to_string()
                            image_path="".to_string()
                        >
                            <p class="pt-2 pb-2">
                               "Cheryl'a bio. Paragraph one."
                            </p>
                            <p class="pb-2">
                                "Cheryl'a bio. Paragraph two."
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
