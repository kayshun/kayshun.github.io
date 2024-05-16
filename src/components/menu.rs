use leptos::*;
use thaw::{Button, ButtonVariant, Drawer, Grid, GridItem, Icon, Space};

use crate::Page;

/// Displays the menu for desktop
#[component]
pub fn WebMenu() -> impl IntoView {
    view! {
        <div class="flex justify-end">
            <a href=Page::Home.path()
               title="Go to home page"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiHome4BuildingsLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">Home</p>
            </a>

            <a href=Page::OurProducts.path()
                title="Our products"
                class="fill-gray-800 hover:fill-orange-600 text-gray-800 hover:text-orange-600 px-3">
                <div class="pl-1"><KayshunSVGIcon/></div>
                <p class="font-oswald text-sm pt-1">Product</p>
            </a>

            <a href=Page::AboutUs.path()
               title="About us"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::IoPeopleCircleOutline height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">About</p>
            </a>

            <a href=Page::Faqs.path()
               title="FAQs"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiQuestionAnswerCommunicationLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">FAQs</p>
            </a>

            <a href=Page::ContactUs.path()
               title="Contact us"
               class="text-gray-800 hover:text-orange-600 px-3">
                <Icon icon=icondata::RiMailBusinessLine height="2em" width="2.5em"/>
                <p class="font-oswald text-sm text-center">Contact</p>
            </a>
        </div>
    }
}

/// Displays the menu for mobile
#[component]
pub fn MobileMenu() -> impl IntoView {
    let show = create_rw_signal(false);

    view! {
        <Button
                variant=ButtonVariant::Link
                on_click=move |_| show.set(true)>
            <Icon
                class="text-gray-800 hover:text-orange-600"
                icon=icondata::RiMenuSystemFill height="3em" width="4em"/>
        </Button>

        <Drawer class="font-oswald text-xl" show width="160px" title="Menu">
            <Space vertical=true>
                <Grid>

                <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::Home.path()
                           title="Go to home page"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::RiHome4BuildingsLine height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">Home</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="pt-2 pb-2 pl-1">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::OurProducts.path()
                            title="Our products"
                            class="fill-gray-800 hover:fill-orange-600 text-gray-800 hover:text-orange-600 px-3">
                            <div class="pl-0.25"><KayshunSVGIcon/></div>
                            <p class="font-oswald text-md">Product</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::AboutUs.path()
                           title="About us"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::IoPeopleCircleOutline height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">About</p>
                        </a>
                </Button>

                    </GridItem>

                    <GridItem class="p-2">

                <Button
                    variant=ButtonVariant::Link
                    on_click=move |_| show.set(false)>
                        <a href=Page::ContactUs.path()
                           title="Contact us"
                           class="text-gray-800 hover:text-orange-600">
                            <Icon icon=icondata::RiMailBusinessLine height="2em" width="2.5em"/>
                            <p class="font-oswald text-md">Contact</p>
                        </a>
                </Button>

                    </GridItem>

                </Grid>
            </Space>
        </Drawer>
    }
}

/// An SVG for the Kayshun icon, for reusability
#[component]
pub fn KayshunSVGIcon() -> impl IntoView {
    view! {
        <svg
         width="30" height="28" viewBox="0 0 794 788"
         preserveAspectRatio="xMidYMid meet">
        <g transform="translate(0.000000,788.000000) scale(0.100000,-0.100000)"
        >
            <path d="M3614 7758 c4 -51 12 -133 17 -183 18 -152 -4 -358 -60 -575 -8 -33
            -31 -94 -79 -216 -25 -60 -116 -238 -143 -275 -10 -15 -19 -30 -19 -33 0 -3
            -25 -42 -55 -85 -31 -44 -77 -111 -103 -149 -82 -119 -116 -164 -162 -219 -72
            -85 -146 -173 -208 -248 -31 -38 -75 -88 -97 -111 -22 -23 -69 -77 -105 -119
            -36 -43 -105 -121 -155 -175 -49 -53 -115 -126 -145 -160 -30 -35 -93 -105
            -140 -156 -112 -123 -160 -179 -160 -189 0 -4 -33 -42 -73 -84 -41 -43 -93
            -103 -118 -135 -24 -32 -57 -75 -74 -96 -16 -21 -52 -68 -80 -106 -27 -37 -64
            -86 -80 -108 -45 -60 -192 -288 -242 -376 -43 -76 -139 -266 -167 -330 -73
            -172 -116 -289 -142 -390 -84 -313 -93 -383 -93 -705 0 -285 4 -318 68 -605
            12 -55 89 -278 129 -375 46 -110 89 -190 206 -381 15 -25 56 -82 90 -126 33
            -44 67 -90 75 -102 26 -38 294 -307 350 -349 377 -292 803 -498 1124 -544 46
            -6 51 16 10 42 -43 28 -217 212 -290 308 -41 54 -138 197 -178 263 -131 218
            -251 561 -285 814 -6 47 -16 104 -21 127 -15 69 -11 436 6 533 8 47 17 99 20
            115 10 65 70 286 90 330 7 17 21 53 30 80 9 28 46 110 82 184 87 178 192 341
            323 501 30 37 70 87 89 111 18 24 171 179 339 344 681 668 806 796 933 956
            198 251 347 519 415 752 8 26 21 71 29 100 62 210 66 587 9 837 -33 142 -93
            301 -171 455 -56 110 -149 260 -187 301 -6 7 -27 33 -46 59 -19 25 -47 61 -62
            80 -16 18 -28 37 -28 41 0 18 -450 464 -468 464 -2 0 -1 -42 2 -92z"/>
            <path d="M5105 5858 c-3 -7 -7 -92 -9 -188 -5 -208 -16 -294 -59 -465 -39
            -157 -184 -447 -305 -611 -28 -38 -64 -86 -79 -108 -28 -38 -248 -275 -307
            -330 -17 -16 -119 -113 -226 -217 -107 -103 -208 -198 -225 -210 -101 -78
            -463 -466 -585 -628 -14 -18 -36 -47 -50 -65 -14 -17 -66 -92 -115 -166 -48
            -74 -96 -146 -105 -160 -24 -37 -143 -284 -173 -360 -35 -91 -86 -256 -98
            -321 -5 -30 -15 -76 -20 -104 -11 -55 -31 -233 -31 -280 1 -44 20 -283 27
            -330 15 -96 46 -203 81 -283 105 -236 146 -299 304 -465 152 -160 273 -237
            480 -309 l125 -43 190 0 c204 1 245 7 412 70 285 108 504 350 591 656 40 140
            42 154 48 302 8 195 -16 358 -83 562 -26 79 -117 328 -153 415 -24 60 -83 254
            -110 365 -42 171 -36 443 14 595 35 106 75 191 75 162 1 -13 7 -38 15 -55 8
            -18 17 -54 20 -82 8 -83 22 -157 40 -215 10 -30 23 -73 29 -95 6 -22 22 -65
            35 -95 33 -78 188 -384 227 -450 18 -30 42 -71 53 -90 94 -159 209 -416 233
            -520 4 -19 13 -53 19 -75 28 -97 33 -153 33 -355 0 -245 -5 -279 -71 -475 -36
            -107 -93 -218 -172 -336 -79 -118 -76 -115 -250 -292 l-155 -159 45 5 c51 5
            215 47 275 70 453 171 863 451 1139 777 84 99 183 228 218 285 51 82 159 287
            185 351 6 16 28 70 49 119 39 97 81 221 89 270 13 76 28 142 35 160 11 27 38
            239 45 350 10 141 8 321 -3 425 -20 180 -32 275 -41 320 -6 25 -15 72 -22 105
            -17 84 -55 233 -75 285 -8 25 -22 65 -29 90 -28 95 -58 174 -107 280 -12 25
            -36 81 -54 124 -30 72 -169 337 -215 411 -97 155 -204 313 -274 405 -19 25
            -53 72 -76 105 -23 33 -71 94 -106 135 -35 41 -87 103 -116 138 -52 64 -201
            228 -281 312 -64 67 -275 262 -305 282 -31 21 -40 22 -46 6z"/>
            </g>
        </svg>
    }
}
