use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to= "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
    #[to= "/"]
    Home,
}