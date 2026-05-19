use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::services::websocket::WebsocketService;
use crate::User;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
                                    u
                                ),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        // Submit on Enter key
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });

        html! {
            <div class="flex w-screen h-screen" style="background: #0f0f23; color: white;">

                <div style="width: 260px; background: #1a1a2e; border-right: 1px solid rgba(255,255,255,0.08); display: flex; flex-direction: column;">
                    <div style="padding: 20px 16px; border-bottom: 1px solid rgba(255,255,255,0.08);">
                        <div style="font-size: 18px; font-weight: 700; color: white;">{"💬 YewChat"}</div>
                        <div style="font-size: 11px; color: rgba(255,255,255,0.3); margin-top: 2px;">{"Powered by Rust & WASM"}</div>
                    </div>
                    <div style="padding: 12px 16px; font-size: 11px; font-weight: 600; color: rgba(255,255,255,0.3); letter-spacing: 1px; text-transform: uppercase;">
                        {format!("Online — {}", self.users.len())}
                    </div>
                    <div style="flex: 1; overflow-y: auto;">
                        {
                            self.users.clone().iter().map(|u| {
                                html!{
                                    <div style="display: flex; align-items: center; padding: 10px 16px; border-radius: 10px; margin: 4px 8px; cursor: pointer; transition: background 0.2s;">
                                        <div style="position: relative;">
                                            <img style="width: 38px; height: 38px; border-radius: 50%; background: #2a2a4a;" src={u.avatar.clone()} alt="avatar"/>
                                            <div style="position: absolute; bottom: 1px; right: 1px; width: 9px; height: 9px; background: #4ade80; border-radius: 50%; border: 2px solid #1a1a2e;"></div>
                                        </div>
                                        <div style="margin-left: 10px;">
                                            <div style="font-size: 14px; font-weight: 500; color: white;">{u.name.clone()}</div>
                                            <div style="font-size: 11px; color: rgba(255,255,255,0.3);">{"active now"}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                <div style="flex: 1; display: flex; flex-direction: column;">

                    // Header
                    <div style="padding: 16px 24px; border-bottom: 1px solid rgba(255,255,255,0.08); display: flex; align-items: center;">
                        <div style="font-size: 20px; margin-right: 10px;">{"💬"}</div>
                        <div>
                            <div style="font-size: 16px; font-weight: 600; color: white;">{"General Chat"}</div>
                            <div style="font-size: 12px; color: rgba(255,255,255,0.3);">{format!("{} members online", self.users.len())}</div>
                        </div>
                    </div>

                    <div style="flex: 1; overflow-y: auto; padding: 20px 24px; display: flex; flex-direction: column; gap: 16px;">
                        {
                            if self.messages.is_empty() {
                                html!{
                                    <div style="text-align: center; color: rgba(255,255,255,0.2); margin-top: 40px;">
                                        <div style="font-size: 40px; margin-bottom: 12px;">{"👋"}</div>
                                        <div style="font-size: 14px;">{"No messages yet. Say hello!"}</div>
                                    </div>
                                }
                            } else {
                                html!{
                                    <>
                                    {
                                        self.messages.iter().map(|m| {
                                            let user = self.users.iter().find(|u| u.name == m.from);
                                            let avatar = user.map(|u| u.avatar.clone()).unwrap_or_default();
                                            html!{
                                                <div style="display: flex; align-items: flex-start; gap: 12px; max-width: 70%;">
                                                    <img style="width: 36px; height: 36px; border-radius: 50%; background: #2a2a4a; flex-shrink: 0;" src={avatar} alt="avatar"/>
                                                    <div>
                                                        <div style="font-size: 13px; font-weight: 600; color: rgba(255,255,255,0.7); margin-bottom: 4px;">{m.from.clone()}</div>
                                                        <div style="background: #1e1e3a; border: 1px solid rgba(255,255,255,0.06); border-radius: 4px 16px 16px 16px; padding: 10px 14px; font-size: 14px; color: rgba(255,255,255,0.85); line-height: 1.5;">
                                                            if m.message.ends_with(".gif") {
                                                                <img style="max-width: 200px; border-radius: 8px;" src={m.message.clone()}/>
                                                            } else {
                                                                {m.message.clone()}
                                                            }
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                    </>
                                }
                            }
                        }
                    </div>

                    <div style="padding: 16px 24px; border-top: 1px solid rgba(255,255,255,0.08); display: flex; align-items: center; gap: 12px;">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Type a message... (Enter to send)"
                            onkeypress={onkeypress}
                            style="flex: 1; background: #1e1e3a; border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 12px 18px; color: white; font-size: 14px; outline: none;"
                            name="message"
                            required=true
                        />
                        <button
                            onclick={submit}
                            style="width: 44px; height: 44px; border-radius: 12px; border: none; background: linear-gradient(135deg, #667eea, #764ba2); display: flex; justify-content: center; align-items: center; cursor: pointer; flex-shrink: 0;"
                        >
                            <svg fill="white" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="width: 18px; height: 18px;">
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}