use irc::{client::Sender, proto::Message};

pub trait Cmd {
    fn command(&self) -> String;
    fn help(&self) -> String;
    fn on_message(&self, sender: &Sender, message: Message, target: &str);
}
