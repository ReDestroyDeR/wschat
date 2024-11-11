use crate::domain::{ChatEvent, Client, Name, SendMessage};
use chrono::Utc;
use log::{error, info, warn};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::Sender;

#[derive(Debug)]
pub struct Chat {
    clients: HashMap<Name, Client>,
    history: Vec<ChatEvent>,
}

impl Chat {
    pub fn new() -> Arc<Mutex<Chat>> {
        Arc::new(Mutex::new(Chat {
            clients: HashMap::default(),
            history: Vec::default(),
        }))
    }
}

pub trait ChatOps {
    fn connect(&self, name: String, sender: Sender) -> Result<Name, Cow<str>>;

    fn disconnect(&self, name: Name);

    fn send_message(&self, by: Name, message: SendMessage);
}

impl ChatOps for Arc<Mutex<Chat>> {
    fn connect(&self, name: String, sender: Sender) -> Result<Name, Cow<str>> {
        let name = Name(name);

        let mut chat = self.lock().unwrap();
        if chat.clients.contains_key(&name) {
            Err(Cow::from("Client already exists"))
        } else {
            let client = Client::new(name, sender);
            let now = Utc::now();
            let _ = chat.broadcast(ChatEvent::Connected {
                client: client.name().clone(),
                at: now,
            });

            match client.whisper(ChatEvent::Hello {
                clients: chat.clients.iter().map(|(name, _)| name.clone()).collect(),
                history: chat.history.clone(),
            }) {
                Err(err) => {
                    error!("Failed to connect. Whisper failed. Err: {}", err);
                    let now = Utc::now();
                    let _ = chat.broadcast(ChatEvent::Disconnected {
                        client: client.name().clone(),
                        at: now,
                    });
                    Err(Cow::from("Failed to whisper Hello event"))
                }
                Ok(()) => {
                    let name = client.name().clone();
                    chat.clients.insert(name.clone(), client);
                    info!("Client connected; {:?}", name);
                    Ok(name.clone())
                }
            }
        }
    }

    fn disconnect(&self, name: Name) {
        let mut chat = self.lock().unwrap();
        chat.clients.remove(&name);
        let now = Utc::now();
        let _ = chat.broadcast(ChatEvent::Disconnected {
            client: name,
            at: now,
        });
    }

    fn send_message(&self, by: Name, message: SendMessage) {
        let mut chat = self.lock().unwrap();
        if !chat.clients.contains_key(&by) {
            warn!("Tried to send a message as an unknown client: {:?}", by)
        } else {
            let now = Utc::now();
            let message = ChatEvent::Message {
                at: now,
                from: by,
                content: message.content,
            };

            info!("Broadcasting message: {:?}", message);

            let _ = chat.broadcast(message.clone());
        }
    }
}

impl Chat {
    fn broadcast(&mut self, event: ChatEvent) {
        self.clients
            .iter()
            .for_each(|(_, client)| client.whisper(event.clone()).unwrap());
        self.history.push(event);
    }
}
