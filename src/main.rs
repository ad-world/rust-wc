use std::{env, fs::{self, File}, io::{self, BufRead}, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    // First, we should parse the input (flag + filename)
    let filename = parse_filename_from_input(args.clone());
    let flag = parse_flag_from_input(args.clone());
    
    // Abstraction for contents of file or stdin
    let mut filecontent: String = String::new();

    // This means we want to use standard input
    if filename.is_err() {
        // Reading content from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(val) => {
                    filecontent.insert(filecontent.len(), '\n');
                    filecontent.insert_str(filecontent.len(), val.as_str());
                }
                Err(_) => break,
            }
        }
        // remove leading new line
        filecontent = (&filecontent.as_str())[1..].to_string();
    } else {
        // This means filename was passed as arg
        match File::open(filename.as_ref().unwrap()) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: File {} not found", filename.as_ref().unwrap());
                exit(1);
            }
        };

        // Read file contents 
        filecontent = fs::read_to_string(filename.as_ref().unwrap()).expect("Could not read file");
    }

    // This means a flag wasn't passed in -> display default results
    if flag.is_err() {
        print!("{} {} {}", get_bytes(&filecontent), get_lines(&filecontent), get_words(&filecontent));
        if filename.is_ok() {
            println!(" {}", filename.as_ref().unwrap());
        } else {
            println!();
        }
        exit(0);
    }  
    
    let unwrapped = filename.as_ref().unwrap();

    // Flag was passed in! Display results based on flag
    if flag.unwrap() == 'c' {
        let bytes = get_bytes(&filecontent);
        println!("{} {}", unwrapped, bytes);
    } else if flag.unwrap() == 'l' {
        let lines = get_lines(&filecontent);
        println!("{} {}", unwrapped, lines);
    } else if flag.unwrap() == 'w' {
        let words = get_words(&filecontent);
        println!("{} {}", unwrapped, words);
    } else if flag.unwrap() == 'm' {
        let chars = get_chars(&filecontent);
        println!("{} {}", unwrapped, chars);
    }
}

fn get_bytes(f: &String) -> usize {
    return f.bytes().len();
}

fn get_lines(f: &String) -> usize {
    return f.split('\n').count()
}

fn get_words(f: &String) -> usize {
    return f.split_whitespace().count();
}

fn get_chars(f: &String) -> usize {
   return f.chars().count();
}

fn get_flag(arg: String) -> Result<char, bool> {
    if arg.len() < 2 {
        return Err(false);
    }

    let flag = arg.chars().nth(1).unwrap();
    match flag {
        'c' => return Ok('c'),
        'l' => return Ok('l'),
        'w' => return Ok('w'),
        'm' => return Ok('m'),
        _ => {
            print_usage();
            return Err(false);
        }
    }
}

fn is_flag(arg: &String) -> bool {
    if arg.chars().nth(0).unwrap() == '-' {
        return true;
    }

    return false;
}

fn print_usage() {
    println!("Usage: rust-wc -[c|l|w|m] <filename>");
}

fn parse_flag_from_input(args: Vec<String>) -> Result<char, bool> {
    if args.len() == 1 {
        // No flag passed in
        return Err(false);
    }

    if args.len() == 2 {
        // args[1] could be flag, or file name
        if is_flag(&args[1]) {
            return get_flag(args[1].clone());
        } else {
            return Err(false);
        }
    }

    if args.len() == 3 {
        // args[1] should be flag in this case
        if is_flag(&args[1]) {
            return get_flag(args[1].clone());
        }
    }
    
    // Should never reach here
    print_usage();
    exit(1);
}

fn parse_filename_from_input(args: Vec<String>) -> Result<String, bool> {
    if args.len() == 1 {
        // No filename passed in
        return Err(false);
    }

    // Filename or flag may be passed in
    if args.len() == 2 {
        if !is_flag(&args[1]) {
            return Ok(args[1].clone());
        } else {
            return Err(false);
        }
    }

    if args.len() == 3 {
        // args[2] should be the filename
        if !is_flag(&args[2]) {
            return Ok(args[2].clone());
        }
    }

    print_usage();
    exit(1);
 }