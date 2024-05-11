pub mod home;
pub mod not_found;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    NotFound,
}
// Provides a means to get the path for a given page.
impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            // The default authenticated page will be "Assignments", if unauthenticated the user is shown the "Login" page.
            Self::Home => "/",
            Self::NotFound => "/*any",
        }
    }
}
