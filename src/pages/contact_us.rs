use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Icon, Input, InputPrefix, Layout, Space, TextArea};
use web_sys::MouseEvent;

/// Default Home Page
#[component]
pub fn ContactUs() -> impl IntoView {
    let name = create_rw_signal(String::from(""));
    let email = create_rw_signal(String::from(""));
    let message = create_rw_signal(String::from(""));

    view! {
        <Title text="Kayshun - Contact"/>
        <Layout class="p-5">

            <Space>
                <Grid class="p-2">
                    <GridItem>
                        <h1 class="font-oswald text-3xl text-gray-800 pb-4">"Contact us"</h1>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-base text-gray-700 pb-2">"Want to find out more about our plans, or join our growing community of teachers?"</p>

                        <p class="text-base text-gray-700">"To do so, please fill out the form below."</p>
                    </GridItem>
                </Grid>
            </Space>

            <Space vertical=true>

                <Grid class="p-3 text-base text-gray-700 max-w-3xl">

                    <GridItem class="pb-2">
                    <label for="name">"Your name: "</label>
                    <Input attr:id="name" attr:name="name" attr:autocomplete="on" value=name>
                        <InputPrefix slot>
                            <Icon icon=icondata::AiUserOutlined/>
                        </InputPrefix>
                    </Input>
                    </GridItem>

                    <GridItem class="pb-2">
                        <label for="email">"Your email: "</label>
                        <Input attr:id="email" attr:name="email" attr:autocomplete="on" value=email>
                            <InputPrefix slot><Icon icon=icondata::RiMailBusinessLine/> </InputPrefix>
                        </Input>
                    </GridItem>

                    <GridItem>
                        <label for="message">"Your message: "</label>
                        <TextArea class="min-h-[300px]" attr:id="message" value=message/>
                    </GridItem>

                    <GridItem class="p-4">
                        <button
                            id="contact_us"
                            name="contact_us"
                            title="Contact Us"
                            class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-110 font-oswald bg-orange-500 hover:bg-orange-800 text-white p-2 rounded"
                            on:click=move |ev: MouseEvent| {
                                ev.prevent_default();
                                // Call Contact API
                            }>
                            "Submit"
                        </button>
                    </GridItem>

                </Grid>

            </Space>

        </Layout>
    }
}
