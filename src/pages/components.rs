use yew::prelude::*;

pub struct Components;

impl Component for Components {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // Construct contents of main section of webpage.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                            <ybc::Box >
                                <p class="title">{ "...titles" }</p>
                             <ybc::Progress value={0.5} />
                            <ybc::Progress />

                            </ybc::Box>
            </>
        }
    }
}
