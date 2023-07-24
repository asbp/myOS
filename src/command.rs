use alloc::{
    collections::LinkedList,
    string::{String, ToString},
    sync::Arc,
};
use shlex::split;
use spin::Mutex;

use crate::{
    print, println,
    util::{event_emitter::EVENT_EMITTER, text_driver::TEXT_DRIVER},
};

pub struct CommandInterpreter {
    command: String,
}

impl CommandInterpreter {
    pub fn new() -> CommandInterpreter {
        return CommandInterpreter {
            command: "".to_string(),
        };
    }

    pub fn listener(&mut self, key: &str) {
        if key == "\n" {
            println!();
            self.parse_command();
            println!();
            print!("> ");
            self.command.clear();
        } else {
            if key == "\u{8}" {
                if self.command.len() > 0 {
                    TEXT_DRIVER.lock().write_str(key);
                    self.command.pop();
                }
            } else {
                TEXT_DRIVER.lock().write_str(key);
                self.command.push_str(key);
            }
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
    println!();
    print!("> ");

    EVENT_EMITTER.lock().on(
        "keyboard_unicode_key",
        Arc::new(|args| {
            let key = args[0];

            COMMAND.lock().listener(key);
        }),
    );
}

lazy_static::lazy_static! {
    pub static ref COMMAND: Mutex<CommandInterpreter> = Mutex::new(CommandInterpreter::new());
}
