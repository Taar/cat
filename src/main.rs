extern crate rand;

use std::io;
use std::io::{BufReader, SeekFrom};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::vec::Vec;
use std::env;
use std::ffi::OsString;
use rand::distributions::{IndependentSample, Range};

struct Cat {
    hunger: i32
}

impl Cat {
    fn new (state: i32) -> Cat {
        let mut hunger: i32 = 0;

        if state > 100 {
            hunger = 100;
        }

        if state > 0 && state <= 100 {
            hunger = state;
        }

        Cat {
            hunger: hunger
        }
    }

    fn is_hungry (&self) -> bool {
        self.hunger <= 0
    }

    fn feed (&mut self) {
        self.hunger = 100; 
    }

    fn drain (&mut self, amount: i32) {
        self.hunger -= amount;
        if self.hunger < 0 {
            self.hunger = 0; 
        }
    }
}

fn main() {
    let data_file: String = String::from("/tmp/cat.txt");
    let mut file = get_or_create_file(data_file).unwrap();
    let state = get_state_from_file(&mut file);
    let mut cat = Cat::new(state);
    let mut action = OsString::new();
    match env::args_os().nth(2) {
        Some(argument) => action.push(argument.as_os_str()),
        None => {}
    }

    match action.into_string().unwrap().as_str() {
        "+treat" => cat.feed(),
        _ => {}
    }

    if cat.is_hungry() {
        println!("Meow.");
    } else {
        match env::args_os().nth(1) {
            Some(path) => {
                print_file_to_stdout(path);
            },
            None => println!("No file path given. Your cat doesn't look impressed."),
        }
        cat.drain(random_number());
    }

    save_state(&mut file, cat.hunger);
}

fn random_number () -> i32 {
    let between = Range::new(10, 70);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

fn save_state (file: &mut File, state: i32) {
    // Note: If you use `set_len` you need to seek the cursor to the beginning
    // of the file. If you don't, it'll start writing at the cursor and fill
    // the rest with 0s up until the cursor. Hard to notice since the characters
    // won't show up if you print the string to stdout.
    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    write!(file, "{}", state).unwrap();
}

fn get_state_from_file (file: &mut File) -> i32 {
    const DEFAULT_VALUE: i32 = 50;

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {
            // Need to trip to remove /r and/or /n from string buffer or it
            // won't parse the file
            match buffer.trim().parse::<i32>() {
                Ok(state) => return state, 
                Err(e) => return DEFAULT_VALUE
            }
        },
        Err(e) => return DEFAULT_VALUE
    }
}

fn get_or_create_file(data_file: String) -> Result<File, io::Error> {
    // set the file to read/write (starting at 0)
    // if it doesn't exist, create it
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(data_file)
}
    
fn read_file (path: OsString) -> io::Result<BufReader<File>> {
    let file = try!(File::open(path));
    let reader: BufReader<File> = BufReader::new(file);
    Ok::<BufReader<File>, io::Error>(reader)
}

fn print_file_to_stdout (path: OsString) {
    match read_file(path) {
        Ok(mut reader) => {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            io::stdout().write(&buffer).unwrap();
        },
        Err(e) => println!("There was an error! {:?}", e),
    }
}
