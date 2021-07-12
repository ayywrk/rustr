use crate::irc::core::Irc;
use crate::enums::{VoidResult, IrcErrors};

impl Irc {
    pub fn handle_command(&mut self, from: &str, channel: &str, argv: Vec<&str>) -> VoidResult {

        for command in self.commands() {
            if command.get_name() == argv[0] {

                if *command.is_private_only() {
                    continue;
                }
                if *command.is_only_masters() {
                    if !self.is_master(from) {
                        return Err(IrcErrors::NotMaster);
                    }
                }

                let argv = argv.into_iter().map(|s| s.to_string()).collect();
                let ret :Vec<String> = match command.get_func() {
                    Some(func) => func(from, channel, argv),
                    None => return Err(IrcErrors::UnknownFunction)
                };

                for line in ret {
                    self.send(&line)?;
                }
                return Ok(());
            }
        }
        Err(IrcErrors::UnknownFunction)
    }

    pub fn handle_private_command(&mut self, from: &str, argv: Vec<&str>) -> VoidResult {
        let nick = from.find("!").map_or(from, |i| &(from)[..i]);

        println!("We in! {} {}", from, argv[0]);
        for command in self.commands() {
            if command.get_name() == argv[0] {

                if *command.is_channel_only() {
                    continue;
                }
                if *command.is_only_masters() {
                    if !self.is_master(from) {
                        return Err(IrcErrors::NotMaster);
                    }
                }

                let argv = argv.into_iter().map(|s| s.to_string()).collect();
                let ret :Vec<String> = match command.get_func() {
                    Some(func) => func(from, nick, argv),
                    None => return Err(IrcErrors::UnknownFunction)
                };

                for line in ret {
                    println!("{}", line);
                    self.send(&line)?;
                }
                return Ok(());
            }
        }
        Err(IrcErrors::UnknownFunction)
    }

}
