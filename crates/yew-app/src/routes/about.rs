use yew::prelude::*;

/// About page
pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="app">
                <header class="app-header">
                    <p>
                        <a
                            class="app-link"
                            href="https://github.com/jetli/create-yew-app"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "Create Yew App" }
                        </a>
                        { ", Set up a modern yew web app by running one command." }
                    </p>
                    <p>
                        { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                    </p>
                </header>
            </div>
        }
    }
}
