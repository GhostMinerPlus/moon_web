use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Home" }</h1>
        }
    }
}
