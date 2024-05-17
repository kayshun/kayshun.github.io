use leptos::*;
use leptos_meta::Title;
use thaw::{Grid, GridItem, Layout, Space};

/// Default Home Page
#[component]
pub fn ContactUs() -> impl IntoView {
    view! {
        <Title text="Kayshun - Contact"/>
        <Layout class="w-full lg:max-w-6xl lg:mx-auto">
            <Space>
                <Grid class="w-full p-4">
                    <GridItem>
                        <h1 class="font-oswald text-3xl pb-4">"Contact us"</h1>
                    </GridItem>
                    <GridItem class="p-4">
                        <p class="text-base pb-2">"Drop us a message to find out more about our plans or join our growing community of teachers."</p>
                    </GridItem>
                </Grid>
            </Space>

            <Space vertical=true>
                // Full width Grid container with gradient background -->
                <Grid class="w-full p-4 text-base text-gray-700 bg-gradient-to-l from-orange-500 from-3% via-orange-400 via-80% to-orange-300">
                    // JS for Google recaptcha -->
                    <script src="https://www.google.com/recaptcha/api.js" async defer></script>
                    // Form submission via the Basin API -->
                    // TODO: Configure account at https://usebasin.com/ -->
                    <form action="#" method="POST">
                        <GridItem class="w-full pb-2">
                            <label for="name">"Your name: "</label>
                        </GridItem>
                        <GridItem class="pb-2">
                            <input id="name" name="name" autocomplete="on" class="w-64 rounded" />
                        </GridItem>

                        <GridItem class="w-full pb-2">
                            <label for="email-address">"Email address: "</label>
                        </GridItem>
                        <GridItem class="w-full pb-2">
                            <input id="email" name="email" autocomplete="on" class="w-64 rounded" />
                        </GridItem>

                        <GridItem class="w-full">
                            <label for="message">"Your message: "</label>
                        </GridItem>
                        <GridItem class="w-full">
                            <textarea id="message" name="message" rows="6" class="w-80 lg:w-3/4 rounded"></textarea>
                        </GridItem>

                        <GridItem class="w-full p-2">
                            <div class="pb-2 g-recaptcha" data-sitekey="6Lew3SMUAAAAAJ82QoS7gqOTkRI_dhYrFy1f7Sqy"></div>
                            <button
                                id="contact_us"
                                name="contact_us"
                                title="Contact Us"
                                type="submit"
                                class="transition ease-in-out delay-50 hover:-translate-y-1 hover:scale-110 font-oswald bg-orange-500 hover:bg-orange-800 text-white p-2 rounded"
                            >
                                "Submit"
                            </button>
                        </GridItem>
                    </form>
                </Grid>
            </Space>
        </Layout>
    }
}
