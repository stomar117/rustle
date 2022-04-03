use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use colored::Colorize;
use crossterm::execute;
use crossterm::cursor::MoveToColumn;
use std::io::Write;

mod readutil;
pub mod read_enum;
use readutil::ReadVector;

pub struct Reader {
    prompt: String,
    validity_vector: Option<Vec<String>>
}

struct KeyEventStore {
    keycode: KeyCode,
    key_mod: KeyModifiers
}

impl Reader {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            validity_vector: Some(Vec::new())
        }
    }
    pub fn get_cmd(&mut self, cmd_vector: Vec<&str>) {
        match &mut self.validity_vector {
            None => {},
            Some(vector) => {
                for token in cmd_vector{
                    vector.push(token.to_string());
                }
            }
        }
    }
    fn get_key_event (&self) -> KeyEventStore {
        enable_raw_mode().unwrap();
        match read() {
            Ok(event) => {
                disable_raw_mode().unwrap();
                match event {
                    Event::Key(key) => KeyEventStore{
                        keycode: key.code,
                        key_mod: key.modifiers
                    },
                    _ => KeyEventStore{
                        keycode: KeyCode::Null,
                        key_mod: KeyModifiers::NONE
                    }
                    
                }
            },
            Err(_) => KeyEventStore{
                keycode: KeyCode::Null,
                key_mod: KeyModifiers::NONE
            }
        }
    }

    fn prettify<'a>(&self, string: &'a str) -> String{
        match &self.validity_vector {
            Some(vector) => {
                let token_vec: Vec<&str> = string.split(" ").collect();
                let mut toke_vec_non_static: Vec<String> = Vec::new();
                for token in token_vec {
                    toke_vec_non_static.push(token.to_string());
                }
                let mut token_vec = toke_vec_non_static;
                let record = vector
                    .iter()
                    .find(|string| string.to_string() == token_vec[0]);
                match record {
                    Some(_finding) => {
                        token_vec[0] = token_vec[0].color("green").to_string();
                    },
                    None => {
                        token_vec[0] = token_vec[0].color("red").to_string();
                    }
                    
                }
                token_vec.join(" ")
            },
            None => string.to_string()
        }
    }

    fn pretty_print(&self, string: String){
        execute!(std::io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
        print!("\r{}{}", &self.prompt, &self.prettify(&string[..]));
        std::io::stdout().flush()
            .expect("failed to flush stdout buffer");
    }

    pub fn read(&self) -> read_enum::ReadEnum{
        let mut readvec = ReadVector::new();
        loop {
            match readvec.join() {
                Ok(string) => {let _ = &self.pretty_print(string);}
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
            let key_event = &self.get_key_event();
            match key_event.key_mod {
                KeyModifiers::NONE => {
                    match key_event.keycode {
                        KeyCode::Char(c) => {
                            match readvec.push(c) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{}", e);
                                    break;
                                }
                            };
                        }
                        KeyCode::Enter => {break;}
                        KeyCode::Backspace => {
                            match readvec.del() {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{}", e);
                                    break;
                                }
                            };
                        }
                        _ => {}
                    }
                }
                KeyModifiers::SHIFT => {
                    match key_event.keycode {
                        KeyCode::Char(c) => {
                            match readvec.push(c.to_ascii_uppercase()) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("{}",e);
                                    break;
                                }
                            };
                        }
                        _ => {}
                    }
                },
                KeyModifiers::CONTROL => {
                    match key_event.keycode {
                        KeyCode::Char(c) => {
                            if c == 'c' {
                                return read_enum::ReadEnum::KeyboardInterrupt;
                            }
                            else if c == 'd' {
                                return  read_enum::ReadEnum::EOFQuit;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        println!();
        match readvec.join() {
            Ok(string) => read_enum::ReadEnum::Command(string),
            Err(e) => {
                eprintln!("{}", e);
                read_enum::ReadEnum::Command(String::new())
            }
        }
    }
}