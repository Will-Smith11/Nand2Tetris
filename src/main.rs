use std::{fs::{self, File}, collections::HashMap, io::{ BufRead, self, Error}, env, ops::Index };
use phf::phf_map;

static C_SYMBOLS: phf::Map<&'static str, &str> = phf_map! {
    "0" => "0101010",
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

fn build_symbol_dict(map: &mut Box<HashMap<String, u16>>)
{
    map.insert("SP".to_string(), 0);
    map.insert("LCL".to_string(), 1);
    map.insert("ARG".to_string(), 2);
    map.insert("THIS".to_string(), 3);
    map.insert("THAT".to_string(), 4);
    map.insert("R0".to_string(), 0);
    map.insert("R1".to_string(), 1);
    map.insert("R2".to_string(), 2);
    map.insert("R3".to_string(), 3);
    map.insert("R4".to_string(), 4);
    map.insert("R5".to_string(), 5);
    map.insert("R6".to_string(), 6);
    map.insert("R7".to_string(), 7);
    map.insert("R8".to_string(), 8);
    map.insert("R9".to_string(), 9);
    map.insert("R10".to_string(), 10);
    map.insert("R11".to_string(), 11);
    map.insert("R12".to_string(), 12);
    map.insert("R13".to_string(), 13);
    map.insert("R14".to_string(), 14);
    map.insert("R15".to_string(), 15);
    map.insert("SCREEN".to_string(), 16384);
    map.insert("KBD".to_string(), 24576);
}

fn cleanup_line(line: &String) -> String
{
    line.replace(" ", "")
}

fn remove_comments(line: &String) -> String
{
    String::from(*line.split("//").collect::<Vec<&str>>().index(0))
}

fn pad_binary(a_bin: &String) -> String
{
    "0".repeat(16-a_bin.len()) + a_bin
}

fn word_a_inst(inst: &String, symbol_map: &mut Box<HashMap<String, u16>>) -> io::Result<u16>
{
    let possible_pos = symbol_map.len() - 7;
    if !symbol_map.contains_key(inst.as_str())
    {
        symbol_map.insert(inst.to_string(), possible_pos as u16);
    }

    Ok(symbol_map.get(inst.as_str()).unwrap().to_owned())
}

fn a_inst(inst: &String, symbol_map: &mut Box<HashMap<String, u16>>) -> io::Result<String>
{
    // start from index 1 to remove
    let converted = inst.parse::<u16>();
    let res = match converted {
        Ok(data) => format!("{:b}", data).to_string(),
        Err(_) =>  format!("{:b}",word_a_inst(inst, symbol_map).unwrap()),
    };

    Ok(res)
}

fn read_and_clean_file(file_name: &String) -> Vec<String>
{
    let file = File::open(file_name).unwrap();

    return io::BufReader::new(file)
        .lines()
        .map(|line | cleanup_line(&line.unwrap()))
        .filter(|line| !line.starts_with("//") && line.len() != 0)
        .map(|line | remove_comments(&line))
        .collect();
}

fn c_two_args(inst: &str, arg1: &str, arg2: &str) -> io::Result<String>
{
    if inst.contains("=")
    {
        Ok("111".to_owned()
        + C_SYMBOLS.get(arg2).unwrap()
        + D_SYMBOLS.get(arg1).unwrap()
        + "000\n")
    }
    else
    {
        Ok("111".to_owned()
        + C_SYMBOLS.get(arg1).unwrap()
        + "000" + JUMP_SYMBOLS.get(arg2).unwrap()
        + "\n")
    }
}

fn c_three_args(arg_vec: &Vec<&str>) -> io::Result<String>
{
    Ok("111".to_owned() 
        + C_SYMBOLS.get(arg_vec[0]).unwrap()
        + D_SYMBOLS.get(arg_vec[1]).unwrap()
        + JUMP_SYMBOLS.get(arg_vec[2]).unwrap()
        + "\n")
}

fn c_inst(inst: &str) -> Result<String, Error>
{
    let pattern = &['=', ';'][..];
    let split = inst.split(pattern).collect::<Vec<&str>>();
    let res = match split.len()
    {
        2 => c_two_args(inst, split[0], split[1]),
        3 => c_three_args(&split),
        _ => panic!("invalid line"),
    };

    Ok(res.unwrap())
}

fn write_file(name: &String, data: &String)
{
    fs::write(name.to_owned() + ".hack", data).unwrap();
}

fn main() {
    // Startup
    let args: Vec<String> = env::args().collect();
    let mut symbol_map: Box<HashMap<String, u16>> = Box::new(HashMap::new());
    build_symbol_dict( &mut symbol_map);
    // Load File
    let file_name = args.index(1);
    let lines: Vec<String> = read_and_clean_file(&file_name);
    let mut result: String = String::new();

    // build symbol map
    for (i, line) in lines.iter().enumerate()
    {
        if line.starts_with("(") && line.ends_with(")")
        {
            symbol_map.insert(line.as_str()[1..line.len()-1].to_string(), i as u16);
        }
    }
    //remove lines
    let cleaned_lines: Vec<&String> = lines
                .iter()
                .filter(|line| !(line.starts_with("(") && line.ends_with(")")))
                .collect();

    // parse instructions 
    for line in &*cleaned_lines
    {
        if line.starts_with("@")
        {
            let no_symbol = String::from(&line[1..]);
            let a_inst = pad_binary(&a_inst(&no_symbol.to_string(), &mut symbol_map).unwrap());
            result.push_str(a_inst.as_str());
            result.push_str("\n");
        }
        else
        {
            result.push_str(&c_inst(line).unwrap_or_else(|err| panic!("{}",err.to_string())));
        }
    }
    write_file(&String::from(*file_name.split(".").collect::<Vec<&str>>().index(0)), &result);
}
