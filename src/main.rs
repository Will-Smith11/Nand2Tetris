use std::{fs::{self, File}, collections::HashMap, io::{ BufRead, BufReader, self, Error}, path::Path, ptr::read, env, ops::Index};
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

fn build_symbol_dict(map: &mut Box<HashMap<&str, u16>>)
{
    map.insert("SP", 0);
    map.insert("LCL", 1);
    map.insert("ARG", 2);
    map.insert("THIS", 3);
    map.insert("THAT", 4);
    map.insert("R0", 0);
    map.insert("R1", 1);
    map.insert("R2", 2);
    map.insert("R3", 3);
    map.insert("R4", 4);
    map.insert("R5", 5);
    map.insert("R6", 6);
    map.insert("R7", 7);
    map.insert("R8", 8);
    map.insert("R9", 9);
    map.insert("R10", 10);
    map.insert("R11", 11);
    map.insert("R12", 12);
    map.insert("R13", 13);
    map.insert("R14", 14);
    map.insert("R15", 15);
    map.insert("SCREEN", 16384);
    map.insert("KBD", 24576);
}

fn cleanup_line(line: &String) -> io::Result<String>
{
    Ok(line.replace(" ", ""))
}

fn remove_comments(line: &String) -> io::Result<String>
{
    Ok(String::from(*line.split("//").collect::<Vec<&str>>().index(0)))
}

fn read_and_clean_file(file_name: &String) -> io::Result<Vec<String>>
{
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|line|
        {
            let cleaned  = cleanup_line(&line.unwrap()).unwrap();
            if !cleaned.starts_with("//") && cleaned.len() != 0
            {
                Some(remove_comments(&cleaned).unwrap());
            }

            None
        })
        .collect();

    Ok(lines)
}

fn main() {
    // Startup
    let args: Vec<String> = env::args().collect();
    let mut symbol_dict: Box<HashMap<&str, u16>> = Box::new(HashMap::new());
    build_symbol_dict( &mut symbol_dict);
    // Load File
    let file_name = args.index(1);

    let lines: Vec<String> = read_and_clean_file(&file_name).unwrap();
    let mut result: String = String::new();

    for (i, line) in lines.iter().enumerate()
    {
        if line.starts_with("(") && line.ends_with(")")
        {
            symbol_dict.insert(&line.as_str()[1..line.len()-1], i as u16);
        }
    }
    //remove lines
    let ac_lines: Vec<&String> = lines
                .iter()
                .filter(|line| !(line.starts_with("(") && line.ends_with(")")))
                .collect();
    
       
}
