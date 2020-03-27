use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use yew::App;
use yew_app::app::App as YewApp;
use yew_app::components::nav::Nav;

#[wasm_bindgen_test]
fn app_has_a_home_page() {
    let app: App<YewApp> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    let learn_yew = yew::utils::document()
        .get_element_by_id("learn_yew")
        .expect("No learn yew anchor or no home page")
        .inner_html();
    assert_eq!(learn_yew, "Learn Yew");
}

#[wasm_bindgen_test]
fn nav_component_has_routes() {
    let app: App<Nav> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    let nav_routes = yew::utils::document().get_elements_by_class_name("app-link");

    assert_eq!(nav_routes.length(), 2);

    let home_route = nav_routes.item(0).expect("No home route").inner_html();
    assert_eq!(home_route, "Home");

    let about_route = nav_routes.item(1).expect("No about route").inner_html();
    assert_eq!(about_route, "About");
}
