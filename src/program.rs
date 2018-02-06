use ::std::fs::{ File };
use ::std::io::{ Read };

pub struct Program {
    pub data: Vec<u8>,
}

impl Program {
    pub fn new (path: &str) -> Program {
        let mut file = File::open(path).expect("File not found!");
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents).expect("Cannot read file!");
        Program {
            data: file_contents,
        }
    }
}