use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav>
                <ul>
                    <li><Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ "Home" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::About} classes="app-link">{ "About" }</Link<AppRoute>></li>
                </ul>
            </nav>
        }
    }
}
