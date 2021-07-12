pub mod irc;
pub mod enums;
pub mod commands;

use crate::irc::core::Irc;
use crate::enums::VoidResult;
use crate::commands::Command;


fn help_func(from: &str, channel: &str, _argv: Vec<String>) -> Vec<String> {
    let ret : Vec<String> = vec![
        format!("PRIVMSG {} :{}: melp?", channel, from)
    ];
    return ret;
}

fn melp_func(_from: &str, channel: &str, _argv: Vec<String>) -> Vec<String> {
    let ret : Vec<String> = vec![
        format!("PRIVMSG {} :\x01ACTION explodes.\x01", channel)
    ];
    return ret;
}

fn main() -> VoidResult {
    let host = "irc.sandngz.net";
    let port :u16 = 6697;
    let ssl = true;

    let nick = "rustr";
    let user = "wrkbot";
    let real = "ayy";

    let channels = &["#ruster"];

    let masters = &["*!*@arabs.ps", "*!*@127.0.0.1"];
    let prefix  = &'~';


    let _ = Irc::new(nick)
                .add_command(
                    Command::new("help")
                        .func(help_func)?
                        .only_masters()?
                        .channel_only()?
                )?
                .add_command(
                    Command::new("melp")
                        .func(melp_func)?
                        .only_masters()?
                )?
                .init_prefix(prefix)?
                .init_masters(masters.to_vec())?
                .set_initial_channels(channels.to_vec())?
                .connect(host, port, ssl)?
                .register(user, real)?
                .run()?;

    Ok(())
}
