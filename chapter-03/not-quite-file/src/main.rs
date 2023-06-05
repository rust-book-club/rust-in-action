//! Simulating files one step at a time.

#![allow(unused_variables)]

use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

trait Read {
    fn read(
        &self,
        save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn new_with_data(
        name: &str,
        data: &Vec<u8>
    ) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            Err(String::from("File must be open for reading"))
        } else {
            let mut tmp = self.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Ok(read_length)
        }
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied.");
        Err(err_msg)
    } else {
        f.state = FileState::Open;
        Ok(f)
    }
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        Err(err_msg)
    } else {
        f.state = FileState::Closed;
        Ok(f)
    }
}

fn main() {
    let mut file = File::new("file.txt");
    let mut buffer = vec![];

    if file.read(&mut buffer).is_err() {
        println!("Error checking is working!")
    }

    file = open(file).unwrap();
    let file_length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{}", text);
}
