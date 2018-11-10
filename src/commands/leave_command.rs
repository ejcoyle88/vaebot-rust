extern crate irc;

use irc::client::prelude::*;
use vaebot_message;
use vaebot_irc_command;
use utilities;

const CMD_STR : &str = "!leave";

fn begins_with_cmd_str(message: &str) -> bool {
    let cmd_len = CMD_STR.len();
    match message.get(..cmd_len) {
        Some(channel) => channel == CMD_STR,
        _ => false
    }
}

pub struct LeaveCommand {

}

impl vaebot_irc_command::VaebotIrcCommand for LeaveCommand {
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
                    Some(raw_chan_name) => {
                        let mut chan_name = raw_chan_name.trim();

                        if chan_name == "" {
                            let target = message.raw_message().response_target();
                            match target {
                                Some(tar) => {
                                    chan_name = tar;
                                },
                                None => ()
                            };
                        }

                        if chan_name == "" {
                            return;
                        }

                        let mut channel : String = "".to_owned();

                        let first_char = chan_name.chars().next();
                        match first_char {
                            Some(f) => if f != '#' { channel.push_str("#") },
                            None => channel.push_str("#")
                        }

                        channel.push_str(chan_name);

                        if channel == "#vaeix" {
                            utilities::respond(&message, "Cannot leave home channel!")
                        } else {
                            utilities::leave(&message, &channel)
                        }
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }
}

pub const LEAVECMDINST : LeaveCommand = LeaveCommand {};