use yew::prelude::*;
use yew_hooks::prelude::*;

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
                    <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90" onclick={ondecrease}>{ "Decrease" }</button>
                    <span class="w-12 inline-block">{ *counter }</span>
                    <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90" onclick={onincrease}>{ "Increase" }</button>
                </p>
                <p>
                    { "Edit " } <code>{ "src/app/home.rs" }</code> { " and save to reload." }
                </p>
            </header>
        </div>
    }
}
