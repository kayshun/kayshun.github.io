use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Layout, Space};

use crate::components::community;
/// Default Home Page
#[component]
pub fn Edu() -> impl IntoView {
    view! {
        <Title text="Kayshun - Product"/>
        <Layout class="w-full lg:max-w-6xl lg:mx-auto">
            <Space>
                <Grid class="w-full text-gray-700">
                    <GridItem class="w-full p-6">
                        <h1 class="font-oswald text-3xl text-gray-800 pb-4">"Introducing "<em>Edu</em>" Kayshun*"</h1>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-base text-gray-700 pb-2">"Our "<em>Edu</em>" web app is designed to simplify your grading process and provide quality, consistent feedback to your students. We understand that timely, focused, high-quality feedback is crucial to students' progress. Numerous studies have demonstrated that students who receive such feedback perform better and are much more engaged. With our app, you can now focus more on teaching and less on administrative tasks."</p>

                        <p class="text-base text-gray-700 pb-2">"Currently, the "<em>Edu</em>" web app offers assessment, grading, and feedback for years 10 to 13, encompassing a broad range of courses for the New Zealand curriculum. Our assessments are meticulously aligned with NZQA NCEA Levels 1 to 3, providing you with accurate and reliable results."</p>

                        <p class="text-base text-gray-700 pb-2">"Soon, the "<em>Edu</em>" web app will provide similar services for other curricula, such as English, Ireland, Australia, and international."</p>

                    </GridItem>

                    <GridItem>
                        <h2 class="font-oswald text-2xl text-gray-800 pb-3">"Super easy to use"</h2>
                    </GridItem>

                    <GridItem class="p-2">

                        <h4 class="font-oswald text-lg text-gray-700 font-bold pb-2">"Walkthough our simple (and silent!) overview."</h4>

                        <div class="pb-2">
                          <script src="https://js.storylane.io/js/v1/storylane.js"></script>
                          <div class="sl-embed" style="position:relative;padding-bottom:calc(56.33% + 27px);width:100%;height:0;transform:scale(1)">
                            <iframe class="sl-demo" src="https://app.storylane.io/demo/yh6ibalapekz" allow="fullscreen" style="position:absolute;top:0;left:0;width:100%;height:100%;border:none;"></iframe>
                          </div>
                        </div>

                    </GridItem>

                    <GridItem>
                        <h2 class="font-oswald text-2xl text-gray-800 pb-3">"Fast turnaround times"</h2>
                    </GridItem>

                    <GridItem class="p-2">

                        <p class="text-base text-gray-700 pb-2">"With our "<em>Edu</em>" web app, students will have their assignments graded to a high level and given detailed feedback on how to improve "<span class="font-bold">"within minutes of submission!"</span>" Also, to see their grades, they will first have to respond to the provided feedback, which makes them think about the strategies they will adopt to improve."</p>

                    </GridItem>

                    <GridItem>
                        <h2 class="font-oswald text-2xl text-gray-800 pb-3">"Integrated, with your data in one place"</h2>
                    </GridItem>

                    <GridItem class="p-2">
                        <p class="text-base text-gray-700 pb-2">"The "<em>Edu</em>" web app collates all the grades, feedback, student responses, and assignment responses in one place so the teacher can easily see their students' progress. The application clearly shows if a student needs help so the teacher can plan for suitable assistance."</p>

                        <p class="text-base text-gray-700 pb-2">"While the "<em>Edu</em>" web app operates as a standalone application, it is also fully integrated into Google Classroom. Students have to just click on the link as they would with any other assignment. The grades, feedback and student responses will be available in the Grade Book."</p>

                    </GridItem>

                    <GridItem>
                        <h2 class="font-oswald text-2xl text-gray-800 pb-3">"See more details"</h2>
                    </GridItem>

                    <GridItem class="p-2">

                        <h4 class="font-oswald text-lg text-gray-700 font-bold pb-2">"Watch a more detailed demo with our founder, Jean."</h4>

                        <iframe class="w-full rounded-xl aspect-video"
                            src="https://www.youtube.com/embed/tgbNymZ7vqY?autoplay=0&mute=1">
                        </iframe>

                    </GridItem>

                </Grid>
            </Space>

            <community::Community/>
        </Layout>
    }
}
