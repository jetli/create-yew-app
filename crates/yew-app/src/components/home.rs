use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="App">
                <header class="App-header">
                    <a 
                        class="App-logo"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                    </a>
                    <p>
                        { "Edit " } <code>{ "src/components/home.rs" }</code> { " and save to reload." }
                    </p>
                    <a
                        class="App-link"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Learn Yew" }
                    </a>
                    <RouterLink text="About" link="/about" classes="App-link"/>
                </header>
            </div>
        }
    }
}