extern crate irc;

use irc::client::prelude::*;

pub fn get_message_creator<'a>(config: &'a Config, client: &'a IrcClient) -> impl Fn(&'a Message) -> VaebotMessage {
    move |raw_message: &'a Message| {
        return VaebotMessage {
            config: config,
            client: client,
            raw_message: raw_message
        }
    }
}

pub struct VaebotMessage<'a> {
    config: &'a Config,
    raw_message: &'a Message,
    client: &'a IrcClient
}

impl<'a> VaebotMessage<'a> {
    pub fn raw_message(&'a self) -> &'a Message {
        self.raw_message
    }

    pub fn config(&'a self) -> &'a Config {
        self.config
    }

    pub fn client(&'a self) -> &'a IrcClient {
        self.client
    }

    pub fn is_owner(&'a self) -> bool {
        let src = self.raw_message.source_nickname();
        match src {
            Some(x) => self.config.is_owner(x),
            _ => false
        }
    }
}