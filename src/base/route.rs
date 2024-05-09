use strum::{AsRefStr, EnumCount, EnumIter};
use stylist::{css, yew::use_style};
use yew::{function_component, html, Html, ToHtml};
use yew_router::{components::Redirect, hooks::use_route, Routable};

const ABOUT: &str = include_str!("../../assets/about.md");
const PROJECTS: &[&str] = &[
    include_str!("../../assets/projects/donut.live.md"),
    include_str!("../../assets/projects/fpsdbg.md"),
];

#[derive(Clone, Debug, Routable, PartialEq, AsRefStr, EnumIter, EnumCount)]
pub enum Route {
    #[at("/about")]
    About,

    #[at("/projects/:id")]
    Projects { id: usize },

    #[at("/resume")]
    Resume,

    #[not_found]
    #[at("/404")]
    NotFound,
}

impl ToHtml for Route {
    fn to_html(&self) -> Html {
        match self {
            Self::About => {
                html! { Html::from_html_unchecked(ABOUT.into()) }
            }
            Self::Projects { id } => {
                let choice = PROJECTS.get(*id).unwrap_or(&"Project doesn't exist.");
                html! { Html::from_html_unchecked((*choice).into()) }
            }
            Self::Resume => {
                html! { <embed src="https://resume.rustychads.com/" width="100%" height="100%"/> }
            }
            Self::NotFound => html! { <Redirect<Route> to={Route::About}/> },
        }
    }
}

#[function_component(Switch)]
pub fn switch() -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    let style = use_style(match route {
        Route::About | Route::Projects { .. } => css!(
            width: 40vw;
            margin: 0 auto;
            margin-top: 20vh;
            padding: 1vw;
            border: var(--border);
            border-radius: var(--border-radius);
            background-color: var(--content-background-color);
            color: var(--text-color);
        ),
        Route::Resume => css!(
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
    });
    html! { <div class={style}> { route.to_html() }</div> }
}
