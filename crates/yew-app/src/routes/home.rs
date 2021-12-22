use yew::prelude::*;

/// Home page
pub struct Home;

impl Component for Home {
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
                        { "Edit " } <code>{ "src/routes/home.rs" }</code> { " and save to reload." }
                    </p>
                    <a
                        id="learn_yew"
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
