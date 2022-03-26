use irc::{client::Sender, proto::Message};

use crate::command::Cmd;

pub struct Hello {}

impl Cmd for Hello {
    fn command(&self) -> String {
        String::from("hello")
    }

    fn help(&self) -> String {
        String::from("Hello command")
    }

    fn on_message(&self, sender: &Sender, message: Message, target: &str) {
        sender
            .send_privmsg(
                target,
                format!("Hello {}!", message.source_nickname().unwrap_or("World!")),
            )
            .expect("Error sending hello");
    }
}
