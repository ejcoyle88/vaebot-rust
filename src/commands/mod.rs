use vaebot_irc_command;

mod hello_command;
mod join_command;

const BUILTIN_CMDS : [&vaebot_irc_command::VaebotIrcCommand; 2] = [
    &hello_command::HICMDINST as &vaebot_irc_command::VaebotIrcCommand,
    &join_command::JOINCMDINST as &vaebot_irc_command::VaebotIrcCommand
];

pub fn get_commands<'a>() -> [&'a vaebot_irc_command::VaebotIrcCommand; 2] {
    BUILTIN_CMDS
}