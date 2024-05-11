use yew::{html, Context, Html, Properties};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub txt: String,
}

pub struct Label {}

impl yew::Component for Label {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <label>{ctx.props().txt.clone()}</label>
        }
    }
}
