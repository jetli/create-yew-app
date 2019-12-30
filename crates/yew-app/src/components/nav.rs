use yew::prelude::*;
use yew_router::prelude::*;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <nav>
                <ul>
                    <li><RouterLink text="Home" link="/" classes="app-link"/></li>
                    <li><RouterLink text="About" link="/about" classes="app-link"/></li>
                </ul>
            </nav>
        }
    }
}
