use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div style="color: white; text-align: center; padding: 40px 20px; 
                    border-radius: 15px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
                    display: flex; justify-content: space-around; align-items: center;
                    flex-direction: row; height: 100vh; width: 100vw; background: linear-gradient(135deg, #ff5722, #ff8f00, #d81b60);">

            <div style="flex: 1; text-align: center; color: black;">
                <h1 style="font-size: 80px; font-weight: bold; margin: 0;">{ "WORD WARS" }</h1>
                <p style="font-size: 30px; margin-top: 10px;">{ "Play with your friends!" }</p>
            </div>

            <div style="flex: 1; display: flex; flex-direction: column; align-items: center;
                        justify-content: center;">
                <Link<Route> to={Route::CreateRoom}>
                    <button style="background-color: #ffc107; color: black; border: none;
                                   padding: 15px 30px; font-size: 20px; margin-bottom: 20px;
                                   cursor: pointer; border-radius: 10px; box-shadow: 0px 3px 6px rgba(0,0,0,0.3);">
                        { "CREATE A ROOM" }
                    </button>
                </Link<Route>>
                <Link<Route> to={Route::JoinRoom}>
                    <button style="background-color: black; color: #ffc107; border: none;
                                   padding: 15px 30px; font-size: 20px; margin-bottom: 20px;
                                   cursor: pointer; border-radius: 10px; box-shadow: 0px 3px 6px rgba(0,0,0,0.3);">
                        { "JOIN A ROOM" }
                    </button>
                </Link<Route>>
            </div>

            <div style="flex: 1; background-color: black; border-radius: 10px; 
                        box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.1); 
                        text-align: left; padding: 20px; color: white;">
                <h2 style="font-size: 28px; margin-bottom: 20px; color: #ffc107;">{ "How to Play" }</h2>
                <ol style="list-style-type: decimal; padding-left: 20px;">
                    <li style="font-size: 18px; margin-bottom: 10px;">
                        { "Click on the 'Create Room' button to start a new game." }
                    </li>
                    <li style="font-size: 18px; margin-bottom: 10px;">
                        { "Share the room code with your friends to join." }
                    </li>
                    <li style="font-size: 18px; margin-bottom: 10px;">
                        { "Gather around a table on Game Night or use online applications for communication during the game." }
                    </li>
                    <li style="font-size: 18px; margin-bottom: 10px;">
                        { "Enjoy!" }
                    </li>
                </ol>
            </div>
        </div>
    }
}
