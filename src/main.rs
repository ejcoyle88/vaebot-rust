extern crate irc;

use irc::client::prelude::*;

mod vaebot_message;
mod commands;
mod vaebot_irc_command;
mod utilities;

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let config = Config {
        nickname: Some("vaeixbot".to_owned()),
        password: Some("oauth:8saumksngfuav8d0blmxv9lw1fg6w7".to_owned()),
        server: Some("irc.chat.twitch.tv".to_owned()),
        channels: Some(vec!["#vaeix".to_owned()]),
        owners: Some(vec!["vaeix".to_owned()]),
        use_ssl: Some(false),
        ..Config::default()
    };
    let client = reactor.prepare_client_and_connect(&config).unwrap();

    client.identify().unwrap();

    reactor.register_client_with_handler(client, move |_client, msg| {
        let cmds = commands::get_commands();
        let create_message = vaebot_message::get_message_creator(&config, &_client);
        let message = create_message(&msg);

        println!("{}", msg.to_string());

        for cmd in cmds {
            if cmd.should_handle_message(&message) {
                cmd.handle_message(&message);
            }
        }

        Ok(())
    });

    reactor.run().unwrap();
}
