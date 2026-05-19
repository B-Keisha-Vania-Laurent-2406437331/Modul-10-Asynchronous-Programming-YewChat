use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone()
        })
    };

    html! {
        <div class="flex w-screen h-screen" style="background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div style="background: rgba(255,255,255,0.05); backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); border-radius: 20px; padding: 48px 40px; min-width: 380px; text-align: center;">
                    <div style="font-size: 48px; margin-bottom: 8px;">{"💬"}</div>
                    <h1 style="color: white; font-size: 28px; font-weight: 700; margin-bottom: 8px;">{"YewChat"}</h1>
                    <p style="color: rgba(255,255,255,0.5); font-size: 14px; margin-bottom: 32px;">{"Real-time chat powered by Rust & WebAssembly"}</p>
                    <div style="display: flex; flex-direction: column; gap: 12px;">
                        <input
                            {oninput}
                            style="background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.15); border-radius: 12px; padding: 14px 18px; color: white; font-size: 15px; outline: none; width: 100%; box-sizing: border-box;"
                            placeholder="Enter your username"
                        />
                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={username.len() < 1}
                                style="width: 100%; padding: 14px; border-radius: 12px; border: none; background: linear-gradient(90deg, #667eea, #764ba2); color: white; font-size: 15px; font-weight: 600; cursor: pointer; transition: opacity 0.2s;"
                            >
                                {"Join Chat →"}
                            </button>
                        </Link<Route>>
                    </div>
                    <p style="color: rgba(255,255,255,0.3); font-size: 12px; margin-top: 24px;">{"No account needed. Just pick a name and start chatting."}</p>
                </div>
            </div>
        </div>
    }
}