#[path="read_util/reader.rs"] mod reader;
use std::ops::Deref;

use crossterm::{execute,terminal::{Clear, ClearType},cursor::MoveTo};
use reader::read_enum::ReadEnum;

fn main() {
    let mut token_reader = reader::Reader::new("input> ");
    token_reader.get_cmd(vec!["clear", "exit", "this", "do"]);
    let mut token_reader = token_reader;
    loop {
        let read_value = token_reader.read();
        match read_value {
            ReadEnum::Command(cmd) => {
                match cmd.deref() {
                    "exit" => {break;}
                    "clear" => {
                        execute!(std::io::stdout(),
                            Clear(ClearType::All),
                            MoveTo(0,0)).expect("failed to clear screen");
                    }
                    _ => {println!("{}", cmd);}
                }
                // if cmd == "exit" {
                //     break;
                // }
                // else if cmd == "clear" {
                //     execute!(std::io::stdout(),
                //         Clear(ClearType::All),
                //         MoveTo(0,0)
                //     )
                //         .expect("Failed to clear screen");
                // }
                // else {
                //     println!("{}", cmd);
                // }
            },
            ReadEnum::KeyboardInterrupt => {
                println!("^C");
            }
            ReadEnum::EOFQuit => {
                println!("^D");
                break;
            }
        }
    }
}
