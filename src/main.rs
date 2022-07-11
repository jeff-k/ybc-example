#![recursion_limit = "512"]

use ybc::NavbarItemTag::{Div, A};
use yew::prelude::*;

mod pages;

use yew_router::prelude::*;

struct Model;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Heros,
    #[at("/components")]
    Components,
    #[at("/elements")]
    Elements,
    #[at("/forms")]
    Forms,
}

#[function_component(Components)]
fn components() -> Html {
    html! {}
}

#[function_component(Elements)]
fn elements() -> Html {
    html! {}
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Heros => html! {
            <pages::heros::Heros />
        },
        Route::Components => html! {
            <pages::components::Components />
        },
        Route::Elements => html! {
            <pages::elements::Elements />
        },
        Route::Forms => html! {
            <pages::forms::Forms />
        },
    }
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                // ybc-element of type Navbar. navbrand, navstart, and navend properties are required and each expect an Html type. The yew html! macro returns this type.
                // The `navbar-burger` section is automatically appended.
                <ybc::Navbar navbrand={self.view_navbrand()} navstart={self.view_navstart()} navend={self.view_navend()} />

                <main>
                <ybc::Container>
                    <Switch<Route> render={Switch::render(switch)} />
                </ybc::Container>
                </main>
                // ybc-element of type Tile. Is a container for all other tiles that compose the main body of webpage.

            </BrowserRouter>
        }
    }
}

impl Model {
    // Contruct the contents of the Navbar brand section and return Html type that navbrand property of Navbar expects.
    // Html type gets tossed into navbrand field of NavbarProps struct. Consult ybc Docs for more info.
    fn view_navbrand(&self) -> Html {
        html! {
            <>
                <ybc::NavbarItem tag={A}>
                <strong>{ "YBC" }</strong>
                </ybc::NavbarItem>
            </>
        }
    }

    // Contruct Navbar navdrop Menu
    fn view_navdrop(&self) -> Html {
        html! {
            <ybc::NavbarDropdown navlink={self.view_navlink()} hoverable=true>
                <ybc::NavbarItem tag={A} href="/components">
                    { "About" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag={A}>
                    { "Jobs" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag={A}>
                    { "Contact" }
                </ybc::NavbarItem>
                <ybc::NavbarDivider />
                <ybc::NavbarItem tag={A}>
                    { "Report an issue" }
                </ybc::NavbarItem>
            </ybc::NavbarDropdown>
        }
    }

    // Contruct the contents of the `navbar-end` section and return Html type that navend property of Navbar expects.
    // Html type gets tossed into navend field of NavbarProps struct. Consult ybc Docs for more info.
    fn view_navend(&self) -> Html {
        html! {
            <ybc::NavbarItem tag={Div}>
                // Create div container for button groups
                <ybc::Buttons>
                    // Button classes property accepts Option<String> type. `is-primary` here provides color styling.
                    <ybc::Button classes={classes!("is-primary")}>
                        <ybc::Icon classes={classes!("fab", "fa-github")} />
                        <strong>{ "Github" }</strong>
                    </ybc::Button>
                </ybc::Buttons>
            </ybc::NavbarItem>
        }
    }

    // Contruct the contents of the `navbar-link` section and return Html type that navlink property of NavbarDropdown expects.
    // Html type gets tossed into navlink field of NavbarDropdownProps struct. Consult ybc Docs for more info.
    fn view_navlink(&self) -> Html {
        html! {
            { "More" }
        }
    }

    // Contruct the contents of the `navbar-start` section and return Html type that navstart property of Navbar expects.
    // Html type gets tossed into navstart field of NavbarProps struct. Consult ybc Docs for more info.
    fn view_navstart(&self) -> Html {
        html! {
            <>
                <ybc::NavbarItem tag={A} href="/heros">
                    { "Heros" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag={A} href="/elements">
                    { "Elements" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag={A} href="/components">
                    { "Components" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag={A} href="/forms">
                    { "Forms" }
                </ybc::NavbarItem>

                { self.view_navdrop() }
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
