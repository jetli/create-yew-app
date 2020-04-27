use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav>
                <ul>
                    <li><RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link" >{ "Home" }</RouterAnchor<AppRoute>></li>
                    <li><RouterAnchor<AppRoute> route=AppRoute::About classes="app-link">{ "About" }</RouterAnchor<AppRoute>></li>
                </ul>
            </nav>
        }
    }
}
