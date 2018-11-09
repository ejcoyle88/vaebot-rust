use vaebot_irc_command;

mod hello_command;

const BUILTIN_CMDS : &[&vaebot_irc_command::VaebotIrcCommand] = &[
    &hello_command::HICMDINST as &vaebot_irc_command::VaebotIrcCommand 
];

pub fn get_commands<'a>() -> &'a[&'a vaebot_irc_command::VaebotIrcCommand] {
    BUILTIN_CMDS
}