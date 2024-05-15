use leptos::*;
use thaw::{Button, ButtonVariant, Grid, GridItem, Image, Modal};

use crate::components::about::BioPanel;
/// The about us page
#[component]
pub fn About() -> impl IntoView {
    let show_john = create_rw_signal(false);

    view! {

        <Grid cols=2 class="font text-base text-gray-700 p-2">

            <GridItem column=2 class="max-w-prose pb-2">
                <p class="font-sans text-base text-gray-700">"Bringing a world-class software service to market is hard! We could not do it on our own."</p>
            </GridItem>

            <GridItem class="">
                <h3 class="font-oswald text-xl text-gray-700 pb-2">"John Dixon"</h3>
                <div class="pt-2">

                    <Button variant=ButtonVariant::Link on_click=move |_| { show_john.set(true) }>

                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            width="100px" height="100px"/>

                    </Button>
                    <Modal
                        title=""
                        width="425px"
                        show=show_john>
                        <BioPanel
                            name="John Dixon".to_string()
                            image_path="".to_string()
                            li_url="https://www.linkedin.com/in/john-dixon-ba54a813b/".to_string()
                        >
                            <p class="pt-2 pb-2">
                               "John's bio. Paragraph one."
                            </p>
                            <p class="pb-2">
                                "John's bio. Paragraph two."
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
