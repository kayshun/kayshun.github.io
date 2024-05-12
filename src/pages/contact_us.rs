use leptos::*;
use thaw::{Grid, GridItem, Icon, Input, InputPrefix, Layout, Space, TextArea};

/// Default Home Page
#[component]
pub fn ContactUs() -> impl IntoView {
    let name = create_rw_signal(String::from(""));
    let email = create_rw_signal(String::from(""));
    let message = create_rw_signal(String::from(""));

    view! {
        <Layout class="p-5">

            <Space>
                <Grid class="p-2">
                    <GridItem>
                        <h1 class="font-oswald text-4xl text-gray-800 pb-4">"Contact us"</h1>
                    </GridItem>
                    <GridItem class="p-2">
                        <p class="text-xl text-gray-700">"Please fill out the form below"</p>
                    </GridItem>
                </Grid>
            </Space>

            <Space vertical=true>

                <Grid class="p-3 text-xl text-gray-700 max-w-4xl">

                    <GridItem>
                    <label for="name">"Your name: "</label>
                    <Input attr:id="name" value=name>
                        <InputPrefix slot>
                            <Icon icon=icondata::AiUserOutlined/>
                        </InputPrefix>
                    </Input>
                    </GridItem>

                    <GridItem>
                        <label for="email">"Your email: "</label>
                        <Input attr:id="email" value=email>
                            <InputPrefix slot><Icon icon=icondata::RiMailBusinessLine/> </InputPrefix>
                        </Input>
                    </GridItem>

                    <GridItem>
                        <label for="message">"Your message: "</label>
                        <TextArea attr:id="message" value=message/>
                    </GridItem>

                </Grid>

            </Space>

        </Layout>
    }
}
