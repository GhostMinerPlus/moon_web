use yew::{html, Html};
use yew_router::prelude::*;

use crate::view::home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

// Public
pub struct Router {}

impl yew::Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::prelude::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::prelude::Context<Self>) -> Html {
        yew::html! {
            <yew_router::BrowserRouter>
                <yew_router::Switch<Route> render={switch} />
            </yew_router::BrowserRouter>
        }
    }
}
