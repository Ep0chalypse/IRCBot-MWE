A minimal working example of a IRC bot written in rust using the [irc crate](https://github.com/aatxe/irc)

The bot connect to EFnet and joins the #rustbot-spam channel and responds to 2 commands:
- hello: 
- - Bot replies with "Hello, nickname!"
- long: 
- - Bot runs a long running function which blocks all threads

## Steps to reproduce issue:
- Clone repo
- cargo build
- cargo run
- Wait for bot to join channel
- Type long
- Immediately type hello

The bot will become unresponsive until the long command finishes.