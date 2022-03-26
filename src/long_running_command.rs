use std::{thread::sleep, time::Duration};

use irc::{client::Sender, proto::Message};

use crate::command::Cmd;

pub struct LongRunning {}

impl Cmd for LongRunning {
    fn command(&self) -> String {
        String::from("long")
    }

    fn help(&self) -> String {
        String::from("A long running command")
    }

    fn on_message(&self, sender: &Sender, _message: Message, target: &str) {
        sender
            .send_privmsg(target, "Long message before sleep()")
            .expect("Error sending long message");
        sleep(Duration::from_secs(10));
        sender
            .send_privmsg(target, "Long message after sleep()")
            .expect("Error sending long message");
    }
}
