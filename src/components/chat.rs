use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;
use yew_agent::{Bridge, Bridged};
use crate::services::event_bus::EventBus;

use crate::{User, services::websocket::WebsocketService};

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
                                    "https://api.dicebear.com/9.x/lorelei/svg"
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
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
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        html! {
    <div style="display: flex; width: 100vw;">
        <div style="position: absolute; top: 12px; right: 12px;">
            <Link<Route> to={Route::About} classes="about-button">
                {"About"}
            </Link<Route>>
        </div>

        <div style="flex: none; width: 224px; height: 100vh; background-color:rgb(159, 190, 252);">
            <div style="font-size: 1.25rem; padding: 12px;">{"Users"}</div>
            {
                self.users.clone().iter().map(|u| {
                    html!{
                        <div style="display: flex; margin: 12px; background-color: white; border-radius: 8px; padding: 8px;">
                            <div>
                                <img src={u.avatar.clone()} alt="avatar" style="width: 48px; height: 48px; border-radius: 9999px;" />
                            </div>
                            <div style="flex-grow: 1; padding: 12px;">
                                <div style="display: flex; justify-content: space-between; font-size: 0.75rem;">
                                    <div>{u.name.clone()}</div>
                                </div>
                                <div style="font-size: 0.75rem; color: #9ca3af;">
                                    {"Hi there!"}
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>

        <div style="flex-grow: 1; height: 100vh; display: flex; flex-direction: column;">
            <div style="width: 100%; height: 56px; border-bottom: 2px solid #d1d5db;">
                <div style="font-size: 1.25rem; padding: 12px;">{"💬 Welcome to YewChat!"}</div>
            </div>
            <div style="width: 100%; flex-grow: 1; overflow: auto; border-bottom: 2px solid #d1d5db;">
                {
                    self.messages.iter().map(|m| {
                        let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                        html!{
                            <div style="display: flex; align-items: flex-end; width: 50%; background-color: rgb(159, 190, 252); margin: 32px; border-radius: 8px 8px 8px 0;">
                                <img src={user.avatar.clone()} alt="avatar" style="width: 32px; height: 32px; border-radius: 9999px; margin: 12px;" />
                                <div style="padding: 12px;">
                                    <div style="font-size: 0.875rem;">{m.from.clone()}</div>
                                    <div style="font-size: 0.75rem; color: #6b7280;">
                                        {
                                            if m.message.ends_with(".gif") {
                                                html! { <img src={m.message.clone()} style="margin-top: 12px;" /> }
                                            } else {
                                                html! { {m.message.clone()} }
                                            }
                                        }
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div style="width: 100%; height: 56px; display: flex; padding: 12px; align-items: center;">
                <input ref={self.chat_input.clone()} type="text" placeholder="Type Your Message Here"
                    name="message" required=true
                    style="flex-grow: 1; padding: 8px 16px; margin-right: 12px; background-color: rgb(209, 221, 245); border-radius: 9999px; outline: none;" />
                <button onclick={submit}
                    style="padding: 12px; background-color: #2563eb; width: 40px; height: 40px; border-radius: 9999px; display: flex; justify-content: center; align-items: center;">
                    <svg fill="#ffffff" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="width: 16px; height: 16px;">
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