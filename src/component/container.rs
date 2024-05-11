use yew::{html, Children, Context, Html, Properties};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    pub children: Children,
}

pub struct Column {}

impl yew::Component for Column {
    type Message = ();

    type Properties = ColumnProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style={"display: flex;flex-direction: column;height: 100%;background-color: white;"}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RowProps {
    children: Children,
}

pub struct Row {}

impl yew::Component for Row {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style={"display: flex;flex-direction: row;width: 100%;background-color: white;"}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
