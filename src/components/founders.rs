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
                "Our founders are a husband-and-wife team with a wealth of experience. While living in pre-Covid China they had a startup, which provided teachers with technical support for STEAM projects. The company offered coding solutions and supported teachers with programming small hardware devices for class projects."
                </div>
            </p>
        </GridItem>

        <GridItem class="ml-8">
            <div class="container max-w-fit columns-2">
                <div class="p-2">
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Jean Clarke"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_one.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            src="/images/jean_headshot.jpg" width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_one>
                        <BioPanel
                            name="Jean Clarke".to_string()
                            image_path="/images/jean_headshot.jpg".to_string()
                            li_url="https://www.linkedin.com/in/jean-clarke-50206b2b2".to_string()
                        >
                            <p class="pt-2 pb-2">
                                "After 24 years igniting young minds in classrooms across the UK, New Zealand, and China, I'm taking my passion for education innovation to a new level. As a lifelong learner and advocate for girls in STEM, I designed and implemented groundbreaking STEAM programmes in multiple schools, fostering a love for science, technology, engineering, arts, and maths."
                            </p>
                            <p class="pb-2">
                                "I'm excited to leverage this experience to revolutionise education with Kayshun. Beyond the classroom, you'll find me conquering triathlons – a testament to my dedication and drive. When not challenging myself physically, I cheer on my two amazing teenage daughters."
                            </p>
                        </BioPanel>
                    </Modal>
                </div>
                <div class="p-2">
                    <h3 class="font-oswald text-xl text-gray-700 p-2">"Mick Clarke"</h3>
                    <Button variant=ButtonVariant::Link on_click=move |_| { show_two.set(true) }>
                        <Image
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-125 h-36 w-36 rounded-full"
                            src="/images/mick_headshot_sepia.jpg" width="100px" height="100px"
                        />
                    </Button>
                    <Modal title="" width="425px" show=show_two>
                        <BioPanel
                            name="Mick Clarke".to_string()
                            image_path="/images/mick_headshot_sepia.jpg".to_string()
                            li_url="https://www.linkedin.com/in/mickclarke138/".to_string()
                            gh_url="https://github.com/avastmick".to_string()
                        >
                            <p class="pt-2 pb-2">
                                "Mick started his working life in off-shore construction in the oil industry. He has lived and worked in multiple countries across the globe and currently lives in New Zealand."
                            </p>
                            <p class="pb-2">
                                "Mick successfully entered technology after obtaining an MSc. in Computer Graphics and AI. Since then, Mick has worked in multiple industries, ranging from media start-ups to large-scale eCommerce, banking, national government programmes, and back to several start-ups."
                            </p>
                            <p class="pb-2">
                                <i>"“Never stop learning; you always know less than you think.”"</i>
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
