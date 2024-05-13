use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Icon, Layout, Space};

use crate::pages::Page;
/// Default Home Page
#[component]
pub fn OurCommunity() -> impl IntoView {
    view! {
        <Title text="Kayshun - Product"/>
        <Layout class="p-5">
            <Space>
                <Grid class="p-2 max-w-prose">

                    <GridItem>
                        <h1 class="font-oswald text-4xl text-gray-800 pb-4">"Edu Kayshun*"</h1>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-lg text-gray-700 pb-2">"Our "<em>Edu</em>" app makes it simple to grade assignments and give quality and consistent feedback to your students. We know that timely, focussed and high-quality feedback to students is vital to their progress. Multiple studies have shown that students who receive such feedback perform better and are much more engaged."</p>

                        <p class="text-lg text-gray-700 pb-2">"With our "<em>Edu</em>" app, students will have their assignments graded to a high level within minutes of submission. Not only that, to see their grades they will first have to respond to the provided feedback, which makes them think about the strategies they will adopt to improve."</p>

                        <p class="text-lg text-gray-700 pb-2">"The "<em>Edu</em>" app collates all the grades, feedback and student responses, as well as the assignment responses in one place so the teacher can easily see their students' progress. If a student needs help the application will show this so the teacher can plan for the suitable assistance."</p>

                        <p class="text-lg text-gray-700">"The "<em>Edu</em>" app is fully integrated into Google Classroom, so the grades, feedback and student responses are available in the Grade Book."</p>


                    </GridItem>

                    <GridItem class="p-2">

                        <p class="pb-2 text-xl font-bold text-gray-700">"Watch our simple (and silent!) overview walkthrough."</p>

                        <div class="p-2 border border-gray-200 rounded-lg ">
                            <div>
                              <script src="https://js.storylane.io/js/v1/storylane.js"></script>
                              <div class="sl-embed" style="position:relative;padding-bottom:calc(56.33% + 27px);width:100%;height:0;transform:scale(1)">
                                <iframe class="sl-demo" src="https://app.storylane.io/demo/yh6ibalapekz" allow="fullscreen" style="position:absolute;top:0;left:0;width:100%;height:100%;border:none;"></iframe>
                              </div>
                            </div>
                        </div>

                    </GridItem>

                    <GridItem class="p-2">

                        <p class="pb-2 text-xl font-bold text-gray-700">"Watch a more detailed demo with our founder, Jean."</p>

                        <div class="p-2 border border-gray-200 rounded-lg ">
                            <iframe class="w-full aspect-video"
                                src="https://www.youtube.com/embed/tgbNymZ7vqY?autoplay=0&mute=1">
                            </iframe>
                        </div>

                    </GridItem>

                </Grid>
            </Space>

            <Space>
                <Grid class="p-2 max-w-prose">

                    <GridItem>
                        <h2 class="font-oswald text-3xl text-gray-800 pb-3">"Join our community"</h2>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"We aim to be a community-driven initiative, with a goal of collectively improving education with digital products that are simple to use and solve real-world issues. To achieve our goals, we need you to help us improve  our products and make them even more usable and useful."</p>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Contact us to find out how you can  be a part of our community and get on our early users team."</p>
                    </GridItem>

                    <GridItem class="p-2">
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600 font-bold pb-2 py-1 px-3">
                            <Icon icon=icondata::RiMailSendBusinessLine height="2.5em" width="3em"/>
                            "Contact us"
                        </a>

                        <p class="text-lg text-gray-500 pt-6"><i>*We all need some edukayshun, init...</i></p>
                    </GridItem>

                </Grid>
            </Space>
        </Layout>
    }
}
