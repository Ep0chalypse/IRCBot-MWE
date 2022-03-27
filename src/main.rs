use std::time::Duration;

use command::Cmd;
use futures::prelude::*;
use hello_command::Hello;
use irc::{
    client::{prelude::Config, Client},
    proto::Command,
};
use long_running_command::LongRunning;
mod command;
mod hello_command;
mod long_running_command;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let config = Config {
        server: Some("irc.choopa.net".to_owned()), // EFnet
        port: Some(6669),
        use_tls: Some(false),
        nickname: Some("RustBot1".to_owned()),
        alt_nicks: vec!["RustBot2,".to_owned(), "RustBot3".to_owned()],
        username: Some("rust".to_owned()),
        realname: Some("Beep Boop".to_owned()),
        channels: vec!["#rustbot-spam".to_owned()],
        version: Some("Rust Bot".to_owned()),
        burst_window_length: Some(4),
        max_messages_in_burst: Some(4),
        ..Config::default()
    };

    let mut client = Client::from_config(config.to_owned()).await?;
    client.identify()?;
    let mut stream = client.stream()?;
    let sender = client.sender();

    let mut commands: Vec<Box<dyn Cmd>> = Vec::new();
    let hello_cmd = Hello {};
    let long_cmd = LongRunning {};
    commands.push(Box::new(hello_cmd));
    commands.push(Box::new(long_cmd));

    while let Some(message) = stream.next().await.transpose()? {
        println!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                // The bot will contain many commands in the future so I avoided using a long if/else or match statement here
                for command in &commands {
                    if msg.eq_ignore_ascii_case(command.command().as_str()) {
                        // This works
                        command.on_message(&sender, message.to_owned(), target);
                    
                        // This doesn't block the thread and the bot remains responsive, but I can't pass variables like sender, message, etc...
                        tokio::spawn(async move {
                            println!("Before sleep()");
                            tokio::time::sleep(Duration::from_secs(10)).await;
                            println!("After sleep()");
                            // This doesn't work when uncommented
                            // command.on_message(&sender, message.to_owned(), target);
                        });  
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
