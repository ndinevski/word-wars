mod components;

use crate::components::home::Home;
use crate::components::create_room::CreateRoom;
use crate::components::join_room::JoinRoom;
use crate::components::footer::Footer;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/create-room")]
    CreateRoom,
    #[at("/join-room")]
    JoinRoom,
    #[at("/")]
    Home,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <style>
                { "
                    * {
                        margin: 0;
                        padding: 0;
                        box-sizing: border-box;
                    }
                    html, body {
                        height: 100%;
                        margin: 0;
                        padding: 0;
                    }
                    #root {
                        height: 100%;
                    }
                " }
            </style>
            <div style="height: 100vh; background: linear-gradient(135deg, #ff5722, #ff8f00, #d81b60);">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
            <Footer />
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::CreateRoom => html! { <CreateRoom /> },
        Route::JoinRoom => html! { <JoinRoom /> },
        Route::Home => html! { <Home /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

