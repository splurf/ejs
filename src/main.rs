mod base;

use base::*;
use strum::{EnumCount, IntoEnumIterator};
use web_sys::window;
use yew::{classes, function_component, html, Html, ToHtml};
use yew_router::{hooks::use_navigator, BrowserRouter, Switch};

#[function_component(NavItems)]
fn nav_bar() -> Html {
    // navigator hook
    let nav = use_navigator().unwrap();

    // convert each route enum (other than NotFound) into an interactive button
    html! {
        <div class={classes!("navbar")}>{
                Route::iter()
            .take(Route::COUNT - 1)
            .map(|route| {
                let nav = nav.clone();
                let route_clone = route.clone();
                let onclick = move |_| nav.push(&route_clone);
                html! { <button {onclick}> { route.as_ref() } </button> }
            })
            .collect::<Html>()
        }</div>
    }
}

#[function_component(AccountLinks)]
fn account_links() -> Html {
    html! {
        <div class={classes!("accounts")}>{
                Accounts::iter()
            .map(|a| html! { <a href={a.as_url()} target="_blank">{a.as_ref()}</a> })
            .collect::<Html>()
        }</div>
    }
}

#[function_component(ThemeToggler)]
fn theme_toggler() -> Html {
    let window = window().unwrap();
    let doc = window.document().unwrap().document_element().unwrap();

    theme::init(&window, &doc);
    let onclick = move |_| {
        theme::toggle(&doc);
    };
    html! { <button class={classes!("theme-toggler")} {onclick}>{"Theme"}</button> }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={|route: Route| route.to_html()} />
            <NavItems/>
            <AccountLinks/>
            <ThemeToggler/>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
