extern crate irc;

use irc::client::prelude::*;
use vaebot_message;
use vaebot_irc_command;
use utilities;

pub struct HiCommand {

}

impl vaebot_irc_command::VaebotIrcCommand for HiCommand {
    fn should_handle_message(&self, message: &vaebot_message::VaebotMessage) -> bool {
        let owned_msg = message.is_owner();
        match &message.raw_message().command {
            &Command::PRIVMSG(ref __, ref msg) => owned_msg && msg == "!hello",
            _ => false
        }
    }
    fn handle_message(&self, message: &vaebot_message::VaebotMessage) {
        match &message.raw_message().command {
            &Command::PRIVMSG(ref __, ref ___) =>
                utilities::respond(&message, "Hi there!"),
            _ => ()
        }
    }
}

pub const HICMDINST : HiCommand = HiCommand {};