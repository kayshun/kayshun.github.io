pub mod about_us;
pub mod contact_us;
pub mod faqs;
pub mod home;
pub mod not_found;
pub mod our_products;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    OurProducts,
    AboutUs,
    Faqs,
    ContactUs,
    NotFound,
}
// Provides a means to get the path for a given page.
impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::OurProducts => "/our_products",
            Self::AboutUs => "/about_us",
            Self::Faqs => "/faqs",
            Self::ContactUs => "/contact_us",

            // Any other route is a 404
            Self::NotFound => "/*any",
        }
    }
}
