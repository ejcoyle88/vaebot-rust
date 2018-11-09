extern crate irc;

use irc::client::prelude::*;
use vaebot_message;
use vaebot_irc_command;
use utilities;

const CMD_STR : &str = "!join ";

fn begins_with_cmd_str(message: &str) -> bool {
    let cmd_len = CMD_STR.len();
    match message.get(..cmd_len) {
        Some(channel) => channel == CMD_STR,
        _ => false
    }
}

pub struct JoinCommand {

}

impl vaebot_irc_command::VaebotIrcCommand for JoinCommand {
    fn should_handle_message(&self, message: &vaebot_message::VaebotMessage) -> bool {
        let owned_msg = message.is_owner();
        match &message.raw_message().command {
            &Command::PRIVMSG(ref __, ref msg) =>
                owned_msg && begins_with_cmd_str(&msg),
            _ => false
        }
    }
    fn handle_message(&self, message: &vaebot_message::VaebotMessage) {
        match &message.raw_message().command {
            &Command::PRIVMSG(ref __, ref msg) => {
                let cmd_len = CMD_STR.len();
                let chan = msg.get(cmd_len..);
                match chan {
                    Some(chan_name) => {
                        let mut channel : String = "#".to_owned();
                        channel.push_str(chan_name);
                        utilities::join(&message, &channel)
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }
}

pub const JOINCMDINST : JoinCommand = JoinCommand {};