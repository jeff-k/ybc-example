use yew::prelude::*;

pub struct Elements;

impl Component for Elements {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <ybc::Hero body={self.body()} classes={classes!("is-medium", "is-primary")}/>
            </>
        }
    }
}

impl Elements {
    fn body(&self) -> Html {
        html! {
            "is-medium, is-primary heroe"
        }
    }
}
