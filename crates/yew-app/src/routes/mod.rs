use yew_router::prelude::*;

pub mod about;
pub mod home;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
    #[to = "/"]
    Home,
}
