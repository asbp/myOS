use alloc::string::{String, ToString};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{print, println};

use super::task::keyboard::ScancodeStream;

pub struct StrReader {
    content: String,
    key_stream: ScancodeStream,
}

impl StrReader {
    pub fn new() -> StrReader {
        StrReader {
            content: "".to_string(),
            key_stream: ScancodeStream::new(),
        }
    }

    pub async fn read(&mut self) -> String {
        return self.read_misc(true).await;
    }

    pub async fn read_misc(&mut self, show_character: bool) -> String {
        self.content.clear();

        while let Some(key) = self.key_stream.next().await {
            if key == "\n" {
                println!();
                break;
            } else {
                if key == "\u{8}" {
                    if self.content.len() > 0 {
                        print!("{key}");
                        self.content.pop();
                    }
                } else {
                    if show_character {
                        print!("{key}");
                    }

                    self.content.push_str(&key);
                }
            }
        }

        return self.content.clone();
    }
}

lazy_static! {
    pub static ref STDIN: Mutex<StrReader> = Mutex::new(StrReader::new());
}
