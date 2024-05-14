use leptos::*;
use thaw::{Grid, GridItem, Icon, Image, Layout, Space};

#[component]
pub fn BioPanel(
    name: String,
    image_path: String,
    children: Children,
    #[prop(optional)] gh_url: String,
    #[prop(optional)] li_url: String,
) -> impl IntoView {
    let gh_is_empty = gh_url.is_empty();
    let li_is_empty = li_url.is_empty();
    view! {
        <Layout class="p-5">

            <Space>

                <Grid cols=1 class="p-2">

                    <GridItem>
                        <h3 class="font-oswald text-xl text-gray-800 pb-4">{name}</h3>
                    </GridItem>

                    <GridItem class="p-2">

                        <Image
                            class="h-36 w-36 rounded-full"
                            src=image_path width="100px" height="100px"/>

                        <div class="pt-4">
                            <Show when=move || !gh_is_empty >
                                <a href=gh_url.clone() target="_blank" class="pt-2 text-gray-600 hover:text-orange-600">
                                    <Icon icon=icondata::FaGithubBrands height="2em" width="2em"/>
                                </a>
                            </Show>

                            <Show when=move || !li_is_empty >
                                <a href=li_url.clone() target="_blank" class="p-2 text-gray-600 hover:text-orange-600">
                                    <Icon icon=icondata::FaLinkedinBrands height="2em" width="2em"/>
                                </a>
                            </Show>
                        </div>

                        {children()}

                    </GridItem>

                </Grid>

            </Space>

        </Layout>
    }
}
