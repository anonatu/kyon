use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};




pub struct CmdLineCounter{
    pub path:String,
    pub count:usize,
}


impl CmdLineCounter{
    pub fn new(path:String)->CmdLineCounter{
        CmdLineCounter{
            path,
            count:0,
        }
    }
    pub fn count_line(&mut self)->Result<usize,Box<dyn Error>>{
        self.count = count_line(&self.path)?;
        Ok(self.count)
    }
}

pub fn count_file_lines(path: &str) -> Result<usize, io::Error> {
    let mut count = 0;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        count += 1;
    }
    Ok(count)
}

pub fn count_lines_in_dir(path: &str) -> Result<usize, io::Error> {
    let mut count = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += count_lines_in_dir(path.to_str().unwrap())?;
        } else {
            count += count_file_lines(path.to_str().unwrap())?;
        }
    }
    Ok(count)
}

pub fn count_line(path: &str)->Result<usize, io::Error>{
    let mut count = 0;
    let mut file = File::open(path)?;
    if file.is_dir(){
        count += count_line(path.to_str().unwrap())?;
    }else{
        count += count_file_lines(path.to_str().unwrap())?;
    }
    Ok(count)
}