use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };

    html! {
        <div class="container text-center">
            <header class="space-y-8">
                <p>
                    <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
                        <img class="w-48 h-48 mx-auto mt-24" src="logo.svg" alt="Yew" />
                    </a>
                </p>
                <p>
                    <a id="learn_yew" class="text-emerald-800 underline" href="https://yew.rs" target="_blank" rel="noopener noreferrer">{ "Learn Yew" }</a>
                </p>
                <p class="space-x-4">
                    <Button onclick={ondecrease}>{ "Decrease" }</Button>
                    <span class="w-12 inline-block">{ *counter }</span>
                    <Button onclick={onincrease}>{ "Increase" }</Button>
                </p>
                <p>
                    { "Edit " } <code>{ "src/app/home.rs" }</code> { " and save to reload." }
                </p>
            </header>
        </div>
    }
}
