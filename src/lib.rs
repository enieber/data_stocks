pub mod model;
use crate::model::{convert_to_data, Data, split_pattern_file};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Lines, Write};
use std::path::Path;

pub fn read_file_to_line(file_path: &str) -> Result<Lines<BufReader<File>>, String> {
    let file = File::open(file_path);
    match file {
        Ok(file_open) => {
            let file_buffer = BufReader::new(file_open);
            return Ok(file_buffer.lines());
        }
        Err(err) => {
            return Err(format!("Error to read file: {}", err));
        }
    }
}

fn create_file_or_open(file_path: &str) -> Result<File, Error> {
    if Path::new(file_path).exists() {
        let file = OpenOptions::new().write(true).append(true).open(file_path);
        return file;
    } else {
        let file = File::create(file_path);
        return file;
    }
}

pub fn write_line(file_path: &str, line: &str) -> Result<String, String> {
    let file = create_file_or_open(file_path);

    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &line);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}


pub fn split_line(line: &str) -> Data {
    let pattern = split_pattern_file();
    let value_data: Vec<&str> = line.split(&pattern).collect();
    let data = convert_to_data(value_data);
    return data;
}

