use std::{fs::{self, File}, collections::HashMap, fmt::Result, io::{BufRead, BufReader, self, Error}, path::Path, ptr::read, env, ops::Index};
use phf::phf_map;

static C_INST: phf::Map<&'static str, &str> = phf_map! {
    "1" => "0111111",
    "-1" => "0111010",
    "D" => "0001100",
    "A" => "0110000",
    "!D" => "0001101",
    "!A" => "0110001",
    "-D" => "0001111",
    "x-A" => "0110011",
    "D+1" => "0011111",
    "A+1" => "0110111",
    "D-1" => "0001110",
    "A-1" => "0110010",
    "D+A" => "0000010",
    "D-A" => "0010011",
    "A-D" => "0000111",
    "D&A" => "0000000",
    "D|A" => "0010101",
    "M" => "1110000",
    "!M" => "1110001",
    "-M" => "1110011",
    "M+1" => "1110111",
    "M-1" => "1110010",
    "D+M" => "1000010",
    "D-M" => "1010011",
    "D&M" => "1000000",
    "M-D" => "1000111",
    "D|M" => "1010101",
};

static D_SYMBOLS: phf::Map<&'static str, &str> = phf_map! {
    "M"=> "001",
    "D"=> "010",
    "MD"=> "011",
    "A"=> "100",
    "AM"=> "101",
    "AD"=> "110",
    "AMD"=> "111",
};

static JUMP_SYMBOLS: phf::Map<&'static str, &str> = phf_map! {
    "JGT"=> "001",
    "JEQ"=> "010",
    "JGE"=> "011",
    "JLT"=> "100",
    "JNE"=> "101",
    "JLE"=> "110",
    "JMP"=> "111",
};

fn read_file<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    
    Ok(io::BufReader::new(file).lines())
}

fn cleanup_line(line: &String) -> io::Result<String>
{
    Ok(line.replace(" ", ""))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut symbol_dict: Box<HashMap<&str, &u16>> = Box::new(HashMap::new());
    let file_name = args.index(1);
    
    if let Ok(lines) = read_file( &file_name) {
        for line in lines {
            if let Ok(mut data) = line {
                let formatted: String = cleanup_line(&data).unwrap();
                println!("{}",formatted);
            }
        }
    }
}
