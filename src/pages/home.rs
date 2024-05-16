use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Layout, Space};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Kayshun - Home"/>
        <Layout>
            <Space>
            <Grid class="w-screen flex-1">

                <Grid>

                    <GridItem class="p-6">
                        <h1 class="font-oswald text-3xl ">"Reigniting education"</h1>
                    </GridItem>

                    <GridItem class="pl-8 p-4 bg-gray-50">
                        <h3 class="font-oswald text-xl pb-4">"Knowing your students' progress needs assessment"</h3>
                        <p class="text-base pb-2 pl-2 max-w-prose">"Consistent assessment and timely, focussed, and high-quality feedback are vital to students' progress. Numerous studies have shown that students who receive such feedback perform better and are much more engaged. But to achieve our goal takes up time!"</p>
                    </GridItem>

                    <GridItem class="pl-16 pt-8 pb-8">
                        <div class="flex pl-4">
                        <p class="text-xl text-orange-400 anim-fade-in-1">
                            <span class="text-2xl">"•"</span>" Make fresh assignments for your students... "</p>
                            <span class="italic anim-fade-in-5">"✔️"</span>
                        </div>
                        <div class="flex pl-4">
                        <p class="text-xl text-orange-400 anim-fade-in-2">
                            <span class="text-2xl">"•"</span>" Quickly grade and return these assignments... "</p>
                            <span class="italic anim-fade-in-5">"✔️"</span>
                        </div>
                        <div class="flex pl-4">
                        <p class="text-xl text-orange-400 anim-fade-in-3">
                            <span class="text-2xl">"•"</span>" Include timely, insightful and actionable feedback to your students... "</p>
                            <span class="italic anim-fade-in-5">"✔️"</span>
                        </div>
                        <div class="flex pl-4">
                        <p class="text-xl text-orange-400 anim-fade-in-4">
                           <span class="text-2xl">"•"</span>" Collate the student data for reports... "</p>
                            <span class="italic anim-fade-in-5">"✔️"</span>
                        </div>
                        <p class="pt-2 pl-8 text-xl italic anim-fade-in-5">"Kayshun can do all that for you! "<span class="text-2xl">"✔️"</span></p>
                    </GridItem>

                    <GridItem class="pl-8 pb-4">
                        <h3 class="font-oswald text-xl ">"Providing the best for our students"</h3>
                    </GridItem>

                    <GridItem class="pl-10 pb-4">

                        <p class="text-base pb-2 max-w-prose">"Our mission is guided by years of hands-on teaching experience in multiple countries worldwide. We know the value of what teachers do, and the needs of our students to excel."</p>

                        <p class="text-base pb-2 max-w-prose">"While we as teachers all strive for the best outcomes for our students, the pressure on our time of grading, providing quality feedback, collating those grades and creating quality reports for school administrators and parents is increasingly eating into the time we need to prepare for the most critical aspect of our profession: teaching in the classroom."</p>

                    </GridItem>

                </Grid>

                <Grid class="bg-gradient-to-r from-40% from-orange-400">

                <GridItem class="pl-8 p-4">
                    <h3 class="font-oswald text-xl text-white">"Making a difference with good technology"</h3>
                </GridItem>

                <GridItem class="pl-10 pb-4">
                    <p class="font-light text-base text-white pb-2 max-w-prose">"We have done something about the situation! We are building "<em>Edu</em>", a simple-to-use web application that makes consistent assessment, grading and giving quality feedback to students easy."</p>

                    <p class="font-light text-base text-white pb-2 max-w-prose">"We believe that technology can make a difference. But only if the technology is easy to use, intuitive, and doesn't add to the cognitive load on teachers. We designed "<em>Edu</em>" to be as simple to use as possible while potentially saving up to 30% of a teacher's time."</p>

                    <p class="font-light text-base text-white pb-2 max-w-prose">"We are testing "<em>Edu</em>" with a group of committed teachers who want to use new technologies and apply them to achieve the best outcomes for their students."</p>

                </GridItem>

                </Grid>

                <GridItem class="pl-8 p-4">
                    <h3 class="font-oswald text-xl ">"Kia ora, New Zealand-based teachers!"</h3>
                </GridItem>

                <GridItem class="pl-10 pb-4">
                    <p class="text-base  pb-2 max-w-prose">"We are proud to be based in New Zealand and call these islands our home. Our initial focus will be bringing "<em>Edu</em>" to Kiwi teachers, who will help us improve here first."</p>

                    <p class="text-base pb-2 max-w-prose">"While New Zealand will get "<em>Edu</em>" first, we are eagerly preparing for our global expansion! Stay tuned for more on our exciting roll-out plans."</p>

                </GridItem>

            </Grid>
            </Space>
        </Layout>
    }
}
