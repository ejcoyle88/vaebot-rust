extern crate irc;

use irc::client::prelude::*;

use vaebot_message;

pub fn respond(message: &vaebot_message::VaebotMessage, response: &str) {
    let target = message.raw_message().response_target();
    match target {
        Some(tar) => {
            match message.client().send_privmsg(tar, response) {
                Err(e) => println!("{}", e),
                _ => ()
            }
        },
        None => ()
    };
}

pub fn join(message: &vaebot_message::VaebotMessage, channel_name: &str) {
    match message.client().send_join(channel_name) {
        Err(e) => println!("{}", e),
        _ => ()
    }
}