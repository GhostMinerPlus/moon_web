use yew::{html, Children, Context, Html, Properties};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub style: String,
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
        let style = format!(
            "display: flex;flex-direction: column;{}",
            ctx.props().style
        );
        html! {
            <div style={style}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RowProps {
    pub style: String,
    pub children: Children,
}

pub struct Row {}

impl yew::Component for Row {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!(
            "display: flex;flex-direction: row;{}",
            ctx.props().style
        );
        html! {
            <div style={style}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
