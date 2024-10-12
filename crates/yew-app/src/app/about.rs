use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let state =
        use_async(async move { fetch_repo(("jetli/create-yew-app".to_string()).clone()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <div class="container text-center">
            <header class="space-y-8">
                <p class="mt-24">
                    <a class="text-emerald-800 underline" href="https://github.com/jetli/create-yew-app" target="_blank" rel="noopener noreferrer"
                    >
                        { "Create Yew App" }
                    </a>
                    { ", Set up a modern yew web app by running one command." }
                </p>
                <p>
                    <Button {onclick}>{ "Load info of this repo" }</Button>
                </p>
                <p>
                    {
                        if state.loading {
                            html! { "Loading, wait a sec..." }
                        } else {
                            html! {}
                        }
                    }
                </p>
                {
                    if let Some(repo) = &state.data {
                        html! {
                            <>
                                <p>{ "Repo name: " }<b>{ &repo.name }</b></p>
                                <p>{ "Repo full name: " }<b>{ &repo.full_name }</b></p>
                                <p>{ "Repo description: " }<b>{ &repo.description }</b></p>
                            </>
                            }
                    } else {
                        html! {}
                    }
                }
                <p>
                    {
                        if let Some(error) = &state.error {
                            match error {
                                Error::DeserializeError => html! { "DeserializeError" },
                                Error::RequestError => html! { "RequestError" },
                            }
                        } else {
                            html! {}
                        }
                    }
                </p>
                <p>
                    { "Edit " } <code>{ "src/app/about.rs" }</code> { " and save to reload." }
                </p>
            </header>
        </div>
    }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    fetch::<Repo>(format!("https://api.github.com/repos/{}", repo)).await
}

/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
    id: i32,
    name: String,
    full_name: String,
    description: String,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use wasm_bindgen_test::*;
    use yew::platform::time::sleep;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;

    #[wasm_bindgen_test]
    async fn _about_page_has_an_app_link() {
        yew::Renderer::<About>::with_root(
            gloo_utils::document().get_element_by_id("output").unwrap(),
        )
        .render();

        sleep(Duration::ZERO).await;

        let app_links = gloo_utils::document().get_elements_by_tag_name("a");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
