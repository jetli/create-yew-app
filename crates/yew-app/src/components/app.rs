use yew::prelude::*;
use yew_router::{prelude::*, Switch, route::Route};

use super::{
    home::Home,
    about::About,
};

pub struct App;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to= "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
    #[to= "/"]
    Home,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::Home => html!{ <Home /> },
                            AppRouter::About => html!{ <About /> },
                            AppRouter::PageNotFound(None) => html!{"Page not found"},
                            AppRouter::PageNotFound(Some(missed_route)) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Some(route.route))
                    })
                />
            </div>
        }
    }
 }