use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Layout};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Kayshun - Home"/>
        <Layout>

            <Grid class="p-4 max-w-prose">

                <GridItem class="p-2">
                    <h1 class="font-oswald text-3xl text-gray-800">"Reigniting education"</h1>
                </GridItem>

                <GridItem class="p-2">
                    <h3 class="font-oswald text-xl text-gray-800">"Providing the best for our students"</h3>
                </GridItem>

                <GridItem class="p-2">

                    <p class="text-base text-gray-700 pb-2">"Our mission is guided by years of hands-on teaching experience in multiple countries across the world. We know that consistent assessment and timely, focussed and high-quality feedback to students is vital to their progress. Multiple studies have shown that students who receive such feedback perform better and are much more engaged."</p>

                    <p class="text-base text-gray-700 pb-2">"While we as teachers all strive for the best outcomes for our students, the pressure on our time of grading, providing quality feedback, collating those grades and creating quality reports for school administrators and parents is increasingly eating into the time we need to prepare for the most important aspect of our profession: teaching in the classroom."</p>

                </GridItem>

                <GridItem class="p-2">
                    <h3 class="font-oswald text-xl text-gray-800">"Making a difference with good technology"</h3>
                </GridItem>

                <GridItem class="p-2">
                    <p class="text-base text-gray-700 pb-2">"We have done something about the situation! We are building "<em>Edu</em>", which is a simple to use web application that makes consistent assessment, grading and giving feedback to students easy."</p>

                    <p class="text-base text-gray-700 pb-2">"We believe that technology can make a difference. But only if the technology is easy to use, intuitive, and doesn't add to the cognitive load on teachers. We designed "<em>Edu</em>" to be as simple to use as possible, while potentially saving upto 30% of a teachers' time."</p>

                    <p class="text-base text-gray-700 pb-2">"We are testing "<em>Edu</em>" with a set of committed teachers who want to use new technologies and apply them for the best outcomes for their students."</p>

                </GridItem>

                <GridItem class="p-2">
                    <h3 class="font-oswald text-xl text-gray-800">"Kia ora, New Zealand-based teachers!"</h3>
                </GridItem>

                <GridItem class="p-2">
                    <p class="text-base text-gray-700 pb-2">"We are proud to be based in New Zealand, and call these islands our home. Our initial focus will be bringing "<em>Edu</em>" to Kiwi teachers, who will help us improve here in New Zealand."</p>

                    <p class="text-base text-gray-700 pb-2">"While New Zealand will get "<em>Edu</em>" first, we have global intentions! Watch this space for more on our roll-out plans."</p>

                </GridItem>

            </Grid>

        </Layout>
    }
}
