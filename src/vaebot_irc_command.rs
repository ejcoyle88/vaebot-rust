use vaebot_message;

pub trait VaebotIrcCommand {
    fn should_handle_message(&self, message: &vaebot_message::VaebotMessage) -> bool;
    fn handle_message(&self, message: &vaebot_message::VaebotMessage) {}
}