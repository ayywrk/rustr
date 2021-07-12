use crate::irc::core::Irc;
use crate::enums::{VoidResult};


impl Irc {
    pub fn on_connect(&mut self, host: &str, port: u16, ssl: bool) -> VoidResult {
        println!("[*] Connected to {}:{} (ssl: {})!", host, port, ssl);
        Ok(())
    }

    pub fn on_disconnect(&mut self) -> VoidResult  {
        println!("[*] Disconnected");
        Ok(())
    }

    pub fn on_join_channels(&mut self, channels: Vec<String>) -> VoidResult  {
        println!("[*] Channels {} joined.", channels.join(","));
        Ok(())
    }

    pub fn on_ping(&mut self, ping_msg: &str) -> VoidResult {
        println!("[*] Ping! ({})", ping_msg);
        self.pong(ping_msg)?;
        Ok(())
    }

    pub fn on_join(&mut self, from: &str, channel: &str) -> VoidResult {
        println!("[*] {} joined {}", from, channel);
        Ok(())
    }

    pub fn on_mode(&mut self, from: &str, channel: &str, mode: &str, target: Vec<&str>) -> VoidResult {
        println!("[*] {} set {} to {} in {}", from, mode, target.join(","), channel);
        Ok(())
    }

    pub fn on_topic(&mut self, from: &str, channel: &str, topic: &str) -> VoidResult {
        println!("[*] {} set the topic of {} to {}", from, channel, topic);
        Ok(())
    }

    pub fn on_nick(&mut self, from: &str, new_nick: &str) -> VoidResult {
        println!("[*] {} is now {}", from, new_nick);
        Ok(())
    }

    pub fn on_notice(&mut self, from: &str, target: &str, message: &str) -> VoidResult {
        println!("[*] Notice from {} to {} -> {}", from, target, message);
        Ok(())
    }

    pub fn on_kick(&mut self, from: &str, channel: &str, who: &str, reason: &str) -> VoidResult  {
        println!("[!] {} kicked {} from {} -> {}", from, who, channel, reason);
        Ok(())
    }

    pub fn on_invite(&mut self, from: &str, who: &str, channel: &str) -> VoidResult  {
        println!("[*] {} Invited {} to {}", from, who, channel);
        Ok(())
    }

    pub fn on_message(&mut self, from: &str, channel: &str, message: &str) -> VoidResult  {
        println!("[*] Message from {} in channel {} -> {}.", from, channel, message);

        match self.prefix() {
            Some(prefix) => match message.chars().next() {
                Some(character) if character == *prefix => {
                    let argv = &message[1..];
                    let argv = argv.split(" ").collect();
                    self.handle_command(from, channel, argv)?;
                },
                Some(_) => (),
                None => ()
            },
            None => ()
        };
        Ok(())
    }

    pub fn on_nick_in_use(&mut self) -> VoidResult  {
        println!("[x] Nick already in use.");
        Ok(())
    }

    pub fn on_part(&mut self, from: &str, channel: &str, message: &str) -> VoidResult  {
        println!("[*] {} part from {} -> {}", from, channel, message);
        Ok(())
    }

    pub fn on_private_message(&mut self, from: &str, message: &str) -> VoidResult  {
        println!("[*] Private Message from {} -> {}.", from, message);

        match self.prefix() {
            Some(prefix) => match message.chars().next() {
                Some(character) if character == *prefix => {
                    let argv = &message[1..];
                    let argv = argv.split(" ").collect();
                    self.handle_private_command(from, argv)?;
                },
                Some(_) => (),
                None => ()
            },
            None => ()
        };

        Ok(())
    }

    pub fn on_quit(&mut self, reason: &str) -> VoidResult  {
        println!("[x] Quit! Reason: {}", reason);
        Ok(())
    }
}
