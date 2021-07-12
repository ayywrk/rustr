use crate::enums::{CommandResult};

pub struct Command {
    name: String,
    func: Option<for<'r, 's> fn(&'r str, &'s str, Vec<String>) -> Vec<String>>,
    only_masters: bool,
    channel_only: bool,
    private_only: bool,
}

impl Command {
    pub fn new(name: &str) -> Command {
        Command {
            name: String::from(name),
            func: None,
            only_masters: false,
            channel_only: false,
            private_only: false

        }
    }


    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_func(&self) -> &Option<for<'r,'s> fn(&'r str, &'s str, Vec<String>) -> Vec<String>> {
        &self.func
    }

    pub fn is_only_masters(&self) -> &bool {
        &self.only_masters
    }

    pub fn is_channel_only(&self) -> &bool {
        &self.channel_only
    }

    pub fn is_private_only(&self) -> &bool {
        &self.private_only
    }



    // public

    pub fn func(mut self, new_func: for<'r,'s> fn(&'r str, &'s str, Vec<String>) -> Vec<String>) -> CommandResult {
        self.func = Some(new_func);
        Ok(self)
    }

    pub fn only_masters(mut self) -> CommandResult {
        self.only_masters = true;
        Ok(self)
    }

    pub fn channel_only(mut self) -> CommandResult {
        self.channel_only = true;
        Ok(self)
    }

    pub fn private_only(mut self) -> CommandResult {
        self.private_only = true;
        Ok(self)
    }

}
