use crate::irc::core::Irc;
use crate::enums::*;


impl Irc {

    pub fn join(&mut self, channels: Vec<String>) -> VoidResult {
        self.send(&format!("JOIN {}", channels.join(",")))?;
        Ok(())
    }

    pub fn part(&mut self, channels: Vec<String>) -> VoidResult {
        self.send(&format!("PART {}", channels.join(",")))?;
        Ok(())
    }

    pub fn pong(&mut self, ping_msg: &str) -> VoidResult {
        self.send(&format!("PONG {}", ping_msg))?;
        Ok(())
    }
}
