use yew::prelude::*;

pub struct Forms;

impl Component for Forms {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // Construct contents of main section of webpage.
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>

            <ybc::Field label={"asdf"}>
            <ybc::Control>
            <ybc::Input name={"zxcv"} update={ctx.link().callback(|_| ())} value={"qwerqwe"} />
            </ybc::Control>
            </ybc::Field>

            <ybc::Field label={"Username"}>
            <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
            <ybc::Input name={"username"} update={ctx.link().callback(|_| ())} classes={classes!("is-success")} value={"bulma"} />
             <ybc::Icon classes={classes!("fas", "fa-user", "is-small", "is-left")} />                 <ybc::Icon classes={classes!("fas", "fa-check", "is-small", "is-right")} />
            </ybc::Control>
            <p class={classes!("help", "is-success")} >
            { "hadgasd dsf" }
            </p>
            </ybc::Field>

            <ybc::Field label={"Email"}>
            <ybc::Control classes={classes!("has-icons-left", "has-icons-right")}>
            <ybc::Input name={"email"} update={ctx.link().callback(|_| ())} classes={classes!("is-danger")} value={"bulma@example.com"} />
             <ybc::Icon classes={classes!("fas", "fa-envelope", "is-small", "is-left")} />                 <ybc::Icon classes={classes!("fas", "fa-exclamation-triangle", "is-small", "is-right")} />
            </ybc::Control>
            <p class={classes!("help", "is-danger")} >
            { "no bad email :(" }
            </p>
            </ybc::Field>

            <ybc::Field label={"Message"}>
            <ybc::Control>
            <ybc::TextArea name={"message"} update={ctx.link().callback(|_| ())} value={""}>

            </ybc::TextArea>
            </ybc::Control>
            </ybc::Field>

            <ybc::Field label="Widgets" classes={classes!("is-grouped")}>
            <ybc::Control>
            <ybc::Checkbox name="checkbox" checked=false update={ctx.link().callback(|_| ())} />
            <ybc::Radio name="radio" value="yes" update={ctx.link().callback(|_| ())} />
            <ybc::Radio name="radio" value="no" update={ctx.link().callback(|_| ())} />



            </ybc::Control>
            </ybc::Field>

            </>
        }
    }
}
