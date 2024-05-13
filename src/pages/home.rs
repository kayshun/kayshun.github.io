use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Image, Layout, Space};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Kayshun - Home"/>
        <Layout>
            <Space>
                <Grid class="p-4 max-w-prose">

                    <GridItem class="p-2">
                        <h1 class="font-oswald text-4xl text-gray-800">"Reigniting education"</h1>
                    </GridItem>

                    <GridItem class="p-2">

                        <p class="text-xl text-gray-700 pb-2">"Our mission is guided by years of hands-on teaching experience across multiple countries across the world. We know that consistent assessment and timely, focussed and high-quality feedback to students is vital to their progress. Multiple studies have shown that students who receive such feedback perform better and are much more engaged."</p>

                        <p class="text-xl text-gray-700 pb-2">"While we as teachers all strive for the best outcomes for our students, the pressure on our time of grading, providing quality feedback, collating those grades and creating quality reports for school administrators and parents is increasingly eating into the time we need to prepare for the most important aspect of our profession: teaching in the classroom."</p>

                        <p class="text-xl text-gray-700 pb-2">"We have done something about this. We are building "<em>Edu</em>", which is a simple to use web application that makes consistent assessment, grading and giving feedback to students easy."</p>

                        <p class="text-xl text-gray-700 pb-2">"We are testing "<em>Edu</em>" with a set of committed teachers who want to use new technologies and apply them for the best outcomes for their students."</p>

                    </GridItem>

                    <GridItem class="">

                            <div class="absolute left-0 w-full h-full" >
                                // Note the position and width allows an image to cross the full container
                                <Image height="800px" width="1000px"
                                    src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg"/>
                            </div>

                    </GridItem>

                </Grid>
            </Space>
        </Layout>
    }
}
