extern crate irc;

use irc::client::prelude::*;

use vaebot_message;

pub fn respond(message: &vaebot_message::VaebotMessage, target: &str, response: &str) {
    match message.client().send_privmsg(target, response) {
        Err(x) => println!("{}", x),
        _ => ()
    }
}