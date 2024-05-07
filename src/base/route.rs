use strum::{AsRefStr, EnumCount, EnumIter};
use yew::{classes, html, Html, ToHtml};
use yew_router::{components::Redirect, Routable};

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

impl ToHtml for Route {
    fn to_html(&self) -> Html {
        match self {
            Self::About => html! { <div class={classes!("content")}>{ "hey Joel :))" }</div> },
            Self::Projects => {
                html! {
                    <div class={classes!("content")}>{
                        "not sure where I wanna keep the text. it's either I store everything in the WASM binary or somewhere \
                         else and just shoot a request to it everytime. if I stored it somewhere else, I wouldn't have to to  \
                         rebuild this everytime then which would be nice"
                    }</div>
                }
            }
            Self::Resume => html! {
                <div class={classes!("resume")}>
                    <embed src="https://resume.rustychads.com/" width="100%" height="100%"/>
                </div>
            },
            Self::NotFound => html! { <Redirect<Route> to={Route::About}/> },
        }
    }
}
