use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize;
use yew::prelude::*;

pub struct Heros;

impl Component for Heros {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // Construct contents of main section of webpage.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                    <>
                        // Tile element used to build grid layouts.
                        // Has optional properties children: Children, classes: Option<String>, tag: String, ctx(context modifier): Option<TileCtx>, vertical: bool, and size: Option<TileSize>
                        <ybc::Tile ctx={Ancestor}>
                        <ybc::Tile vertical=true size={TileSize::Eight}>
                            <ybc::Tile>
                        <ybc::Tile ctx={Parent} vertical=true>
                        <ybc::Notification classes={classes!("notification", "is-primary")}>
                                        <p class="title">{ "Vertical..." }</p>
                                        <p class="subtitle">{ "Top tile" }</p>
                                    </ybc::Notification>
                                    <ybc::Box >
                                        <p class="title">{ "...titles" }</p>
                                     <ybc::Progress value={0.5} />
                                    <ybc::Progress />

                                    </ybc::Box>
                                </ybc::Tile>
                                <ybc::Tile ctx={Parent}>
                                    <ybc::Tile ctx={Child} tag="article" classes={classes!("notification", "is-info")}>
                                        <p class="title">{ "Middle tile" }</p>
                                        <p class="subtitle">{ "With an image" }</p>
                                        <figure class="image is-4by3">
                                            <img src="https://bulma.io/images/placeholders/640x480.png" />
                                        </figure>
                                    </ybc::Tile>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent}>
                                <ybc::Tile ctx={Child} tag="article" classes={classes!("notification", "is-danger")}>
                                    <p class="title">{ "Wide tile" }</p>
                                    <p class="subtitle">{ "Aligned with the right tile" }</p>
                                    <div class="content">
                                        <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin ornare magna eros, eu pellentesque tortor vestibulum ut. Maecenas non massa sem. Etiam finibus odio quis feugiat facilisis." }</p>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} tag="article" classes={classes!("notification", "is-success")}>
                                <div class="content">
                                    <p class="title">{ "Tall title" }</p>
                                    <p class="subtitle">{ "With even more content" }</p>
                                    <div class="content">
                                    <ybc::Button classes={classes!("is-primary", "is-warning")}>
                                        { "butan" }
                                    </ybc::Button>


                                    <ybc::Box>

                                    <ybc::Delete />
                                        <ul>
        <li>{ "asdf"} </li>
        </ul>
                                    <ybc::Progress value={0.5} />
                                    </ybc::Box>

                                        <p>
                                            {
                                                "Suspendisse varius ligula in molestie lacinia. Maecenas varius eget ligula a sagittis. Pellentesque interdum, nisl nec interdum maximus, augue diam porttitor lorem, et sollicitudin felis neque sit amet erat. Maecenas imperdiet felis nisi, fringilla luctus felis hendrerit sit amet. Aenean vitae gravida diam, finibus dignissim turpis. Sed eget varius ligula, at volutpat tortor."
                                            }
                                        </p>
                                    </div>
                                </div>
                            </ybc::Tile>
                        </ybc::Tile>
                        </ybc::Tile>
                    </>
                }
    }
}
