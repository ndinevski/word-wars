use crate::components::hero::Hero;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Hero />
    }
}