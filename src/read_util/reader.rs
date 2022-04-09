use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::{cursor::MoveToColumn, execute};
use colored::{Colorize, ColoredString};
use std::io::Write;
use std::collections::HashMap;
use std::ops::Deref;

mod readutil;
pub mod read_enum;
use readutil::ReadVector;

// Reader struct contains all the required objects for both parsing the string and navigation
pub struct Reader {
    prompt: String,
    validity_vector: Option<Vec<String>>,
    color_map: HashMap<String, String>
}

// An easy to use interface for returning keystrokes
struct KeyEventStore {
    keycode: KeyCode,
    key_mod: KeyModifiers
}

// Painter function which would update the color map if zny color change or token change is detected
fn paint(color_map: &mut HashMap<String, String>, token: &str, color: &str, overlay: bool) -> ColoredString{
    let tempmap = color_map.clone();
    if color_map.is_empty() {
        color_map.insert(String::from(token), String::from(color));
    }
    // println!("{:?}", color_map);
    for (key, value) in tempmap {
        match token.to_string().find(&key) {
            Some(0) => {
                if !overlay {
                    if value != color || token != key {
                        color_map.remove(&key);
                        color_map.insert(token.to_string(), color.to_string());
                    }
                }
            },
            _ => {
                match key.find(token) {
                    Some(0) => {
                        if !overlay {
                            if value != color || token != key {
                                color_map.remove(&key);
                                color_map.insert(token.to_string(), color.to_string());
                            }
                        }
                        else {
                            return token.color(value);
                        }
                    }
                    _ => {
                        if overlay {
                            return token.color(color);
                        }
                        if !overlay {
                            color_map.insert(token.to_string(), color.to_string());
                        }
                    }
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
                let mut token_vec_non_static: Vec<String> = Vec::new();
                for token in token_vec {
                    token_vec_non_static.push(token.to_string());
                }
                let token_vec: Vec<&str> = string.split(" ").collect();
                let mut check: u8 = 0;
                for (idx, &token) in token_vec.iter().enumerate() {
                    if !token.is_empty() {
                        if check == 0 {
                            let init_record = vector
                                .iter()
                                .find(|string| string.to_string() == token.to_string());
                            match init_record {
                                Some(_) => {
                                    token_vec_non_static[idx] = paint(&mut self.color_map, token.deref(), "green", overlay).to_string();
                                },
                                None => {
                                    token_vec_non_static[idx] = paint(&mut self.color_map, token.deref(), "red", overlay).to_string();
                                }
                            }
                            check = 1;
                        }
                        if check == 1 {
                            // println!("{}", check);
                        }
                    }

                }
                token_vec_non_static.join(" ")
            },
            None => string.to_string()
        }
    }

    fn pretty_print(&mut self, readvector: &ReadVector){
        let mut _string = String::new();
        let prompt = &self.prompt.clone();
        match readvector.join() {
            Ok(rstring) => {_string = rstring;}
            Err(e) => panic!("{}",e)
        }
        execute!(
            std::io::stdout(),
            Clear(ClearType::CurrentLine),
            MoveToColumn(0)
        ).expect("Failed to clear buffer");

        std::io::stdout().flush().unwrap();
        print!("\r{}{}", prompt, &mut self.prettify(_string.deref(), false));
        print!("\r{}{}", prompt, &mut self.prettify(readvector.join_lt().deref(), true));
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