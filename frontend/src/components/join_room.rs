use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::Route;

#[function_component(JoinRoom)]
pub fn join_room() -> Html {
    let player_name = use_state(|| String::new());
    let room_code = use_state(|| String::new());
    let player_name_error = use_state(|| String::new());
    let room_code_error = use_state(|| String::new());

    let on_player_name_input = {
        let player_name = player_name.clone();
        let player_name_error = player_name_error.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                player_name.set(input.value());
                player_name_error.set(String::new());
            }
        })
    };

    let on_room_code_input = {
        let room_code = room_code.clone();
        let room_code_error = room_code_error.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                room_code.set(input.value());
                room_code_error.set(String::new());
            }
        })
    };

    let onsubmit = {
        let player_name = player_name.clone();
        let room_code = room_code.clone();
        let player_name_error = player_name_error.clone();
        let room_code_error = room_code_error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let mut has_error = false;

            if player_name.is_empty() {
                player_name_error.set("Player Name cannot be empty!".to_string());
                has_error = true;
            }

            if room_code.is_empty() {
                room_code_error.set("Room Code cannot be empty!".to_string());
                has_error = true;
            }

            if !has_error {
                web_sys::console::log_1(&format!("Player Name: {}", *player_name).into());
                web_sys::console::log_1(&format!("Room Code: {}", *room_code).into());
            }
        })
    };

    html! {
        <div style="position: relative; height: 100vh; display: flex; flex-direction: column;
                justify-content: center; align-items: center;
                background: linear-gradient(135deg, #ff5722, #ff8f00, #d81b60);">

            <div style="position: absolute; top: 20px; left: 20px;">
                <Link<Route> to={Route::Home}>
                    <button style="background-color: #ffc107; color: black; border: none;
                                   padding: 15px 30px; font-size: 20px; margin-bottom: 20px;
                                   cursor: pointer; border-radius: 10px; box-shadow: 0px 3px 6px rgba(0,0,0,0.3);">
                        { "Back to Home" }
                    </button>
                </Link<Route>>
            </div>

            <div style="background-color: black; color: #ffc107; padding: 40px 20px; border-radius: 15px;
                        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); max-width: 400px; width: 100%; text-align: center;">
                <h1>{ "Join a Room" }</h1>

                <form onsubmit={onsubmit} style="display: flex; flex-direction: column; align-items: center;">
                    <input
                        type="text"
                        placeholder="Enter Player Name"
                        value={(*player_name).clone()}
                        oninput={on_player_name_input}
                        style="padding: 10px; margin: 10px 0; font-size: 18px; border-radius: 5px;
                            border: 2px solid #ffc107; width: 300px; text-align: center;
                            background-color: black; color: #ffc107;"
                    />
                    if !player_name_error.is_empty() {
                        <p style="color: #ff6b6b; font-size: 14px; margin-top: -8px;">{ (*player_name_error).clone() }</p>
                    }

                    <input
                        type="text"
                        placeholder="Enter Room Code"
                        value={(*room_code).clone()}
                        oninput={on_room_code_input}
                        style="padding: 10px; margin: 10px 0; font-size: 18px; border-radius: 5px;
                            border: 2px solid #ffc107; width: 300px; text-align: center;
                            background-color: black; color: #ffc107;"
                    />
                    if !room_code_error.is_empty() {
                        <p style="color: #ff6b6b; font-size: 14px; margin-top: -8px;">{ (*room_code_error).clone() }</p>
                    }

                    <button type="submit" style="background-color: #ffc107; color: black; border: none;
                                                 padding: 15px 30px; font-size: 19px; margin-top: 20px;
                                                 cursor: pointer; border-radius: 10px; box-shadow: 0px 3px 6px rgba(0,0,0,0.3);">
                        { "Join Room" }
                    </button>
                </form>
            </div>
        </div>
    }
}
