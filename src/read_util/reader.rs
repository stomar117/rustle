use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::{cursor::MoveToColumn, execute};
use colored::{Colorize, ColoredString};
use std::io::Write;
use std::collections::HashMap;

mod readutil;
pub mod read_enum;
use readutil::ReadVector;

pub struct Reader {
    prompt: String,
    validity_vector: Option<Vec<String>>,
    color_map: HashMap<String, String>
}

struct KeyEventStore {
    keycode: KeyCode,
    key_mod: KeyModifiers
}

fn paint(color_map: &mut HashMap<String, String>, token: &str, color: &str, overlay: bool) -> ColoredString{
    let tempmap = color_map.clone();
    if color_map.is_empty() {
        color_map.insert(String::from(token), String::from(color));
    }
    for (key, value) in tempmap {
        // println!("key: {} value: {}", key, value);
        match token.to_string().find(&key) {
            Some(0) => {
                let tmpvalue = value.clone();
                color_map.insert(String::from(token), value);
                color_map.remove(&key);
                if overlay {
                    println!("2");
                    return token.color(&tmpvalue[..]);
                }
            },
            _ => {
                match key.find(token) {
                    Some(0) => {
                        if overlay {
                            println!("1");
                            return token.color(value);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    match color_map.get_mut(token) {
        Some(default_key) => {
            if color != default_key {
                color_map.remove(token);
                color_map.insert(String::from(token), String::from(color));
            }
        }
        _ => {}
    }
    token.color(color)
}

impl Reader {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            validity_vector: Some(Vec::new()),
            color_map: HashMap::new()
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

    fn prettify<'a>(&mut self, string: &'a str, overlay: bool) -> String{
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
                        token_vec[0] = paint(&mut self.color_map, &token_vec[0][..], "green", overlay).to_string();
                    },
                    None => {
                        token_vec[0] = paint(&mut self.color_map, &token_vec[0][..], "red", overlay).to_string();
                    }
                    
                }
                token_vec.join(" ")
            },
            None => string.to_string()
        }
    }

    fn pretty_print(&mut self, readvector: &ReadVector){
        let mut _string = String::new();
        let prompt = &self.prompt.clone();
        match readvector.join() {
            Ok(rstring) => {_string = rstring}
            Err(e) => panic!("{}",e)
        }
        execute!(std::io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).expect("Failed to clear buffer");
        std::io::stdout().flush().unwrap();
        print!("\r{}{}", prompt, &mut self.prettify(&_string[..], false));
        print!("\r{}{}", prompt, &mut self.prettify(&readvector.join_lt()[..], true));
        std::io::stdout().flush()
            .expect("failed to flush stdout buffer");
        if readvector.is_empty() {
            self.color_map.clear();
            return;
        }
    }

    pub fn read(&mut self) -> read_enum::ReadEnum{
        let mut readvec = ReadVector::new();
        loop {
            match readvec.join() {
                Ok(_string) => {let _ = &self.pretty_print(&readvec);}
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
                            match readvec.del_back() {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{}", e);
                                    break;
                                }
                            };
                        }
                        KeyCode::Delete => {
                            match readvec.del() {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{}", e);
                                    break;
                                }
                            }
                        }
                        KeyCode::Left => {readvec.move_back().unwrap();}
                        KeyCode::Right => {readvec.move_front().unwrap();}
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