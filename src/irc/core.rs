use openssl::ssl::{SslMethod, SslConnector, SslVerifyMode};
use std::net::TcpStream;
use std::io::{Write, Read};
use regex::Regex;

use crate::{
    enums::{
        StatusCode,
        Stream,
        IrcResult,
        VoidResult
    },
    commands::Command
};
pub use crate::enums::IrcErrors;

const MAX_LEN :usize = 512 - "\r\n".len();


pub struct Irc {
    nick: String,
    stream: Option<Stream<TcpStream>>,
    user: Option<String>,
    real: Option<String>,
    prefix: Option<char>,
    masters: Vec<String>,
    channels: Vec<String>,
    commands: Vec<Command>,
    carry: String,
}



impl Irc {

    pub fn new(nick: &str) -> Irc {
        Irc {
            nick: String::from(nick),
            stream: None,
            user: None,
            real: None,
            prefix: None,
            masters: Vec::new(),
            channels: Vec::new(),
            commands: Vec::new(),
            carry: String::from("")
        }
    }

    // Getters / Setters

    pub fn nick(&self) -> &String {
        &self.nick
    }
    pub fn set_nick(&mut self, new_nick: String) {
        self.nick = new_nick;
    }

    pub fn user(&self) -> &Option<String> {
        &self.user
    }
    pub fn set_user(&mut self, new_user: String) {
        self.user = Some(new_user);
    }

    pub fn real(&self) -> &Option<String> {
        &self.real
    }
    pub fn set_real(&mut self, new_real: String) {
        self.real = Some(new_real);
    }

    pub fn prefix(&self) -> &Option<char> {
        &self.prefix
    }
    pub fn set_prefix(&mut self, new_prefix: char) {
        self.prefix = Some(new_prefix);
    }

    pub fn masters(&self) -> &Vec<String> {
        &self.masters
    }
    pub fn set_masters(&mut self, new_masters: Vec<String>) {
        self.masters = new_masters;
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
    pub fn set_commands(&mut self, new_commands: Vec<Command>) {
        self.commands = new_commands;
    }

    pub fn channels(&self) -> &Vec<String> {
        &self.channels
    }
    pub fn set_channels(&mut self, new_channels: Vec<String>) {
        self.channels = new_channels;
    }


    // pirvate


    fn write(&mut self, data: &[u8]) -> Result<(), IrcErrors>  {
        match self.stream.as_mut().unwrap() {
            Stream::Tcp(stream_) => {
                stream_.write(data).unwrap();
            },
            Stream::Tls(stream_) => {
                stream_.write(data).unwrap();
            }
        }
        Ok(())
    }

    fn read(&mut self) -> Result<Vec<String>, IrcErrors> {
        let mut received: Vec<u8> = self.carry.as_bytes().to_vec();
        let mut rx_bytes = [0u8; 128];

        loop {
            let bytes_read = match self.stream.as_mut().unwrap() {
                Stream::Tls(stream_) => stream_.read(&mut rx_bytes).unwrap(),
                Stream::Tcp(stream_) => stream_.read(&mut rx_bytes).unwrap()
            };

            received.extend_from_slice(&rx_bytes[..bytes_read]);

            if bytes_read < 128 {
                break;
            }
        }

        let str_data = String::from_utf8(received).unwrap();

        let mut lines :Vec<String> = str_data.split("\r\n").map(|s| s.to_string()).collect();
        self.carry = lines.pop().unwrap();

        Ok(lines)
    }



    // public

    pub fn add_command(mut self, command: Command) -> IrcResult {
        self.commands.push(command);
        Ok(self)
    }

    pub fn is_master(&self, host: &str) -> bool {
        for m in &self.masters {
            let re_str = m.replace(".", "\\.").replace("*", ".*");
            let re = Regex::new(&re_str).unwrap();
            if re.is_match(host) {
                return true;
            }
        }
        return false;
    }

    pub fn connect(mut self, host: &str, port: u16, ssl: bool) -> IrcResult {
        let stream = match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(stream) => stream,
            Err(_) => return Err(IrcErrors::TcpStreamConnect)
        };


        if ssl {
            let connector = match SslConnector::builder(SslMethod::tls()) {
                Ok(mut connector) => {
                    connector.set_verify(SslVerifyMode::NONE);
                    connector.build()
                },
                Err(_) => return Err(IrcErrors::SslConnectorBuild)
            };


            let ssl_stream = match connector.connect(host, stream) {
                Ok(ssl_stream) => ssl_stream,
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err(IrcErrors::SslStreamConnect);
                }
            };

            self.stream = Some(Stream::Tls(ssl_stream));

        } else {
            self.stream = Some(Stream::Tcp(stream));
        }
        self.on_connect(host, port, ssl)?;
        Ok(self)
    }

    pub fn init_prefix(mut self, prefix: &char) -> IrcResult {
        self.prefix = Some(*prefix);
        Ok(self)
    }

    pub fn init_masters(mut self, masters: Vec<&str>) -> IrcResult {
        self.masters = masters.into_iter().map(|s| s.to_string()).collect();
        Ok(self)
    }

    pub fn set_initial_channels(mut self, channels: Vec<&str>) -> IrcResult {
        self.channels = channels.into_iter().map(|s| s.to_string()).collect();
        Ok(self)
    }

    pub fn send(&mut self, msg: &str) -> VoidResult {
        let trimmed = msg.replace("\r", "").replace("\n", "");
        let mut s :String = match trimmed.len() {
            len if len>510  => String::from(&trimmed[..MAX_LEN]),
            len if len<=510 => String::from(trimmed),
            _ => return Err(IrcErrors::Unknown)
        };
        s.push_str("\r\n");
        self.write(s.as_bytes())?;
        Ok(())
    }

    pub fn register(mut self, user: &str, real: &str) -> IrcResult {
        self.user = Some(user.to_owned());
        self.real = Some(real.to_owned());

        self.send(&format!("USER {} 0 * :{}", user, real))?;
        self.send(&format!("NICK {}", self.nick))?;

        Ok(self)
    }

    pub fn run(&mut self) -> VoidResult {

        loop {
            let lines = match self.read() {
                Ok(lines) => lines,
                Err(err) => return Err(err)
            };

            for line in lines {
                match self.handle_line(&line) { // this is shit, I know
                    Ok(_) => (),
                    Err(_) => ()
                };
            }
        }

    }

    fn handle_line(&mut self, line: &str) -> VoidResult {

        let items: Vec<_> = line.split(" ").collect();

        if items[0] == "PING" {
            self.on_ping(&(items[1])[1..])?;
            return Ok(());
        }

        if items[0] == ":Closing Link:" {
            self.on_quit(&items[3..].to_vec().join(" ").to_owned())?;
            return Ok(());
        }

        let sc :StatusCode = match items[1].parse::<StatusCode>() {
            Ok(val) => val,
            Err(_) => StatusCode::UNKNOWN
        };

        // to get nick
        //let nick = &(items[0].find("!").map_or(items[0], |i| &(items[0])[..i]).to_owned())[1..];

        let from = &(items[0])[1..];

        match sc {
            StatusCode::ERR_NICKNAMEINUSE   => self.on_nick_in_use()?,
            StatusCode::RPL_WELCOME         => self.join(self.channels.clone())?,
            StatusCode::JOIN                => {
                let channel = &(items[2])[1..];
                self.on_join(from, channel)?;
            },
            StatusCode::PART                => {
                let channel = items[2];
                let message  = &(&items[3..].to_vec().join(" "))[1..];
                self.on_part(from, channel, message)?;
            },
            StatusCode::MODE                => {
                let channel = items[2];
                let mode    = items[3];
                let targets: Vec<_> = items[4..].to_vec();
                self.on_mode(from, channel, mode, targets)?;
            },
            StatusCode::NICK                => {
                let new_nick = &(items[2])[1..];
                self.on_nick(from, new_nick)?;
            },
            StatusCode::NOTICE              => {
                let target  = items[2];
                let message = &(&items[3..].to_vec().join(" "))[1..];
                self.on_notice(from, target, message)?;
            },
            StatusCode::KICK                => {
                let channel = items[2];
                let who     = items[3];
                let reason  = &(&items[4..].to_vec().join(" "))[1..];
                self.on_kick(from, channel, who, reason)?;
            },
            StatusCode::INVITE              => {
                let who     = items[2];
                let channel = &(&items[3])[1..];
                self.on_invite(from, who, channel)?;
            },
            StatusCode::TOPIC               => {
                let channel = items[2];
                let topic   = &(&items[3..].to_vec().join(" "))[1..];
                self.on_topic(from, channel, topic)?;
            },
            StatusCode::PRIVMSG             => {
                let channel = items[2];
                let message = &(&items[3..].to_vec().join(" "))[1..];
                match channel.chars().next() {
                    Some(character) => match character {
                        '#'     => self.on_message(from, channel, message)?,
                        _       => self.on_private_message(from, message)?
                    },
                    None => return Err(IrcErrors::Unknown)
                };
            },
            StatusCode::UNKNOWN             => return Err(IrcErrors::Unknown),
            _                               => ()
        }

        Ok(())
    }

}
