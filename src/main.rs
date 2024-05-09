mod base;

use base::*;
use strum::{EnumCount, IntoEnumIterator};
use stylist::{
    css,
    yew::{use_style, Global},
};
use web_sys::window;
use yew::{function_component, html, Html};
use yew_router::{hooks::use_navigator, BrowserRouter};

#[function_component(AccountLinks)]
fn account_links() -> Html {
    let style = use_style!(
        display: flex;
        flex-direction: row;
        gap: 1.2vw;
        align-items: center;
        justify-content: center;
        top: 0;
        position: fixed;
        width: 100vw;
        height: 2vh;
        padding: 1vh;
        background-color: var(--accounts-background-color);
        opacity: 0.8;

        > a {
            color: var(--accounts-a-color);
        }
    );

    html! {
        <div class={style}>{
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

    let style = use_style!(
        display: flex;
        align-items: center;
        justify-content: center;
        top: 0.75vh;
        right: 0.4vw;
        position: fixed;
        width: 3vw;
        height: 2.5vh;
        border-radius: var(--border-radius);
        background-color: var(--accounts-background-color);
        border: var(--border);
        color: var(--text-color);
        opacity: 0.8;

        :hover {
            background-color: var(--navbar-button-hover-background-color);
        }
    );

    html! { <button class={style} {onclick}>{"Theme"}</button> }
}

#[function_component(NavItems)]
fn nav_bar() -> Html {
    // navigator hook
    let nav = use_navigator().unwrap();

    let style = use_style!(
        display: flex;
        flex-direction: column;
        align-items: center;
        top: 20vh;
        left: 0;
        bottom: 20vh;
        position: fixed;
        width: 7vw;
        height: 42vh;
        border: var(--border);
        border-radius: var(--border-radius);
        background-color: var(--navbar-background-color);
        opacity: 0.7;

        button {
            width: 6.5vw;
            height: 4vh;
            margin-top: 1.4vh;
            border: var(--border);
            border-radius: var(--border-radius);
            background-color: var(--navbar-button-background-color);
            color: var(--text-color);
        }

        &button:hover {
            background-color: var(--navbar-button-hover-background-color);
        }
    );

    // convert each route enum (other than NotFound) into an interactive button
    html! {
        <div class={style}>{
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

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Global css={css! (
                --body-background-color: #e2e2e2;
                --border: 0.1vh solid #4b4b4b;
                --border-radius: 0.8vh;
                --text-color: rgb(0, 0, 0);
                --navbar-background-color: #d4d4d4;
                --navbar-button-background-color: #c7c7c7;
                --navbar-button-hover-background-color: #cecece;
                --accounts-background-color: #bbbbbb;
                --accounts-a-color: rgb(29, 29, 29);
                --content-background-color: #d8d5d5;

                [data-theme="light"] {
                    --body-background-color: #e2e2e2;
                    --border: 0.1vh solid #4b4b4b;
                    --text-color: rgb(0, 0, 0);
                    --navbar-background-color: #d4d4d4;
                    --navbar-button-background-color: #c7c7c7;
                    --navbar-button-hover-background-color: #cecece;
                    --accounts-background-color: #bbbbbb;
                    --accounts-a-color: rgb(29, 29, 29);
                    --content-background-color: #d8d5d5;
                }

                [data-theme="dark"] {
                    --body-background-color: #090a0a;
                    --border: 0.1vh solid #b4b4b4;
                    --text-color: rgb(230, 230, 230);
                    --navbar-background-color: #26272b;
                    --navbar-button-background-color: #181a1b;
                    --navbar-button-hover-background-color: #3f3f3f;
                    --accounts-background-color: #181a1b;
                    --accounts-a-color: rgb(177, 177, 177);
                    --content-background-color: #0f1011;
                }

                body {
                    margin: 0;
                    overflow: hidden;
                    background-color: var(--body-background-color);
                }
            )}/>
            <Switch/>
            <AccountLinks/>
            <ThemeToggler/>
            <NavItems/>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
