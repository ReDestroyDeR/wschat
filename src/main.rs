use crate::chat::{Chat, ChatOps};
use crate::domain::Name;
use log::{error, info, warn};
use std::sync::{Arc, Mutex};
use ws::{listen, CloseCode, Handler, Message, Request, Response, Sender};

mod chat;
mod domain;

fn main() {
    colog::init();

    let chat = Chat::new();

    listen("0.0.0.0:8080", |out| ChatHandler::new(out, chat.clone())).expect("Error in Main Loop");
}

struct ChatHandler {
    sender: Sender,
    name: Option<Name>,
    chat: Arc<Mutex<Chat>>,
}

impl ChatHandler {
    fn new(sender: Sender, chat: Arc<Mutex<Chat>>) -> Self {
        Self {
            sender,
            name: None,
            chat,
        }
    }
}

impl Handler for ChatHandler {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if let Some(name) = &self.name {
            if let Some(msg) = msg.as_text().ok() {
                match serde_json::from_str(msg) {
                    Ok(msg) => self.chat.send_message(name.clone(), msg),
                    Err(err) => error!("Failed deserializing message; {}", err.to_string()),
                }
            }
        } else {
            warn!(
                "Received message from not initialized connection; {}",
                self.sender.connection_id()
            );
        }

        Ok(())
    }

    fn on_close(&mut self, _close_code: CloseCode, _reason: &str) {
        if let Some(name) = &self.name {
            self.chat.disconnect(name.clone());
        }
    }

    fn on_request(&mut self, req: &Request) -> ws::Result<Response> {
        info!("Received request");

        if let Some(name) = req
            .resource()
            .split(|a| a == '/')
            .last()
            .map(|s| s.trim())
            .filter(|&s| !s.is_empty())
        {
            match self.chat.connect(name.to_owned(), self.sender.clone()) {
                Ok(name) => {
                    self.name = Some(name);
                    Response::from_request(req)
                }
                Err(error) => {
                    warn!("Failed connecting; Error={}", error);
                    Ok(Response::new(412, error.clone(), error.as_bytes().to_vec()))
                }
            }
        } else {
            let err = "No name provided in path";
            warn!("No name provided in path");
            Ok(Response::new(412, err, err.as_bytes().to_vec()))
        }
    }
}
