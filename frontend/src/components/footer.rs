use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer style="position: fixed; bottom: 0; left: 0; right: 0; background-color: black;
                       color: white; text-align: center; padding: 10px 0; font-size: 16px;">
                { "Created by ndinevski - " }
            <a href="https://github.com/ndinevski" target="_blank" style="color: #ffc107; text-decoration: none;">
                { "Open Source on GitHub" }
            </a>
        </footer>
    }
}
