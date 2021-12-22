use yew::prelude::*;

/// About page
pub struct About;

impl Component for About {
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;
    use yew::start_app;

    #[wasm_bindgen_test]
    fn about_page_has_an_app_link() {
        start_app::<About>();

        let app_links = gloo_utils::document().get_elements_by_class_name("app-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
