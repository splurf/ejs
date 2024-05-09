use strum::{AsRefStr, EnumCount, EnumIter};
use stylist::{css, yew::use_style, StyleSource};
use yew::{function_component, html, Html};
use yew_router::{components::Redirect, hooks::use_route, Routable};

#[derive(Clone, Debug, Routable, PartialEq, AsRefStr, EnumIter, EnumCount)]
pub enum Route {
    #[at("/about")]
    About,

    #[at("/projects")]
    Projects,

    #[at("/resume")]
    Resume,

    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    #![allow(non_upper_case_globals)]
    pub fn use_style(&self) -> StyleSource {
        match self {
            Self::About | Self::Projects => css!(
                width: 40vw;
                margin: 0 auto;
                margin-top: 20vh;
                padding: 1vw;
                text-align: center;
                border: var(--border);
                border-radius: var(--border-radius);
                background-color: var(--content-background-color);
                color: var(--text-color);
            ),
            Self::Resume => css!(
                    width: 48vw;
                    height: 90vh;
                    margin: 0 auto;
                    margin-top: 6vh;
                    padding: 0.8vh;
                    text-align: center;
                    border: var(--border);
                    border-radius: var(--border-radius);
                    background-color: var(--content-background-color);
                    color: var(--text-color);
            ),
            _ => css!(),
        }
    }

    pub const fn content(&self) -> &'static str {
        match self {
            Self::About => "hey Joel :))",
            Self::Projects => "not sure where I wanna keep the text. it's either I store everything in the WASM binary or somewhere \
                               else and just shoot a request to it everytime. if I stored it somewhere else, I wouldn't have to to \
                               rebuild this everytime then which would be nice",
            _ => unreachable!()
        }
    }
}

#[function_component(Switch)]
pub fn switch() -> Html {
    let route = if let Some(route) = use_route::<Route>() {
        route
    } else {
        return Html::default();
    };

    let style = use_style(route.use_style());

    match route {
        Route::About | Route::Projects => html! { <div class={style}>{ route.content() }</div> },

        Route::Resume => html! {
            <div class={style}>
                <embed src="https://resume.rustychads.com/" width="100%" height="100%"/>
            </div>
        },
        Route::NotFound => html! { <Redirect<Route> to={Route::About}/> },
    }
}
