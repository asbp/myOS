use alloc::{
    collections::LinkedList,
    string::{String, ToString},
};
use shlex::split;
use spin::Mutex;

use crate::{print, println, util::str_reader::STDIN};

pub struct CommandInterpreter {
    command: String,
}

impl CommandInterpreter {
    pub fn new() -> CommandInterpreter {
        return CommandInterpreter {
            command: "".to_string(),
        };
    }

    pub async fn start(&mut self) {
        loop {
            self.command.clear();
            print!("> ");
            let my_cmd = STDIN.lock().read().await;

            self.command.push_str(&my_cmd);
            self.parse_command();
            println!();
        }
    }

    fn parse_command(&mut self) {
        let mut binding = split(&self.command).map(|vec| {
            let linked_list: LinkedList<String> = vec.into_iter().collect();
            linked_list
        });

        let parsed = binding.as_mut();

        if let Some(command) = parsed {
            self.exec(command);
        }
    }

    fn exec(&mut self, cmd: &mut LinkedList<String>) {
        if let Some(program_name) = cmd.pop_front() {
            if program_name == "whoami" {
                println!("github");
            } else if program_name == "greet" {
                greet(cmd);
            } else if program_name == "add" {
                add(cmd);
            } else {
                println!("Unknown command: {:?}", self.command);
            }
        }
    }
}

pub fn greet(args: &mut LinkedList<String>) {
    let name = args.pop_front();

    if let Some(my_name) = name {
        println!("Hello, {}!", my_name);
    } else {
        println!("Tell me your name, please.")
    }
}

pub fn add(args: &mut LinkedList<String>) {
    let mut total: isize = 0;

    for item in args {
        if let Ok(number) = item.parse::<isize>() {
            total += number;
        }
    }

    println!("{total}");
}

pub fn start_command_interpreter() {
    executor::run(async { COMMAND.lock().start().await });
}

lazy_static::lazy_static! {
    pub static ref COMMAND: Mutex<CommandInterpreter> = Mutex::new(CommandInterpreter::new());
}
