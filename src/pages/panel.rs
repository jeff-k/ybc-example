use yew::prelude::*;

pub struct Panel;

impl Component for Panel {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut fs = vec![];
        let fs_icon = html! { <ybc::Icon classes={classes!("fas", "fa-upload")} /> };
        html! {
            <ybc::Columns>
                <ybc::Column classes={classes!("is-three-fifths")}>
                <ybc::Field>
                <ybc::Control classes={classes!("is-loading")}>
                    <ybc::TextArea name="info" update={ctx.link().callback(|_| ())} value={"input"} classes={classes!("has-fixed-size", "is-danger")}>
                    </ybc::TextArea>
                    </ybc::Control>

                    <p class={classes!("help", "is-danger")}>
                    { "keep going!!" }
                    </p>
                </ybc::Field>
                </ybc::Column>
                <ybc::Column>
                    <ybc::File classes={classes!("is-fullwidth")} has_name={Some("filenaeme")} update={ctx.link().callback(|_| ())} files={fs}  name="files" selector_icon={fs_icon}>

                    </ybc::File>
                </ybc::Column>
            </ybc::Columns>
        }
    }
}

impl Panel {
    fn body(&self) -> Html {
        html! {
            "is-medium, is-primary heroe"
        }
    }
}
