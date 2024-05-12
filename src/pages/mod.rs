pub mod about_us;
pub mod contact_us;
pub mod home;
pub mod not_found;
pub mod our_products;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    NotFound,
    AboutUs,
    ContactUs,
    OurProducts,
}
// Provides a means to get the path for a given page.
impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::AboutUs => "/about_us",
            Self::ContactUs => "/contact_us",
            Self::OurProducts => "/our_products",
            Self::NotFound => "/*any",
        }
    }
}
