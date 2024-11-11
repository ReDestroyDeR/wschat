use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ws::{Message, Sender};

#[derive(Eq, PartialEq, Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Name(pub String);

#[derive(Debug)]
pub struct Client {
    name: Name,
    sender: Sender,
}

impl Client {
    pub fn new(name: Name, sender: Sender) -> Self {
        Self { name, sender }
    }

    pub fn whisper(&self, event: ChatEvent) -> ws::Result<()> {
        self.sender.send(event)
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "kind")]
pub enum ChatEvent {
    Message {
        from: Name,
        content: String,
        at: DateTime<Utc>,
    },
    Hello {
        clients: Vec<Name>,
        history: Vec<ChatEvent>,
    },
    Connected {
        client: Name,
        at: DateTime<Utc>,
    },
    Disconnected {
        client: Name,
        at: DateTime<Utc>,
    },
}

impl Into<Message> for ChatEvent {
    fn into(self) -> Message {
        Message::Text(
            serde_json::to_string(&self).expect("Somehow we fucked up serializing ChatEvent"),
        )
    }
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub content: String,
}
