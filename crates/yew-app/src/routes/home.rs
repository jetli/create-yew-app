use yew::prelude::*;

/// Home page
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

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header class="app-header">
                    <a
                        class="app-logo"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                    </a>
                    <p>
                        { "Edit " } <code>{ "src/components/home.rs" }</code> { " and save to reload." }
                    </p>
                    <a
                        class="app-link"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Learn Yew" }
                    </a>
                </header>
            </div>
        }
    }
}
