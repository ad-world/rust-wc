use std::{env, fs::{self, File}, io::{BufRead, BufReader}, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust-wc -[c|l|w|m] <filename>");
        return;
    }

    if !is_flag(args[1].clone()) {
        // default option, file is first argument

        let filename = &args[1];
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: File {} not found", filename);
                exit(1);
            }
        };

        let content = fs::read_to_string(filename).unwrap();

        println!("{} {} {} {}", get_bytes(&content), get_lines(&content), get_words(&content), filename);
        exit(0);
    }

    let flag = get_flag(args[1].clone());
    if flag.is_err() {
        exit(1);
    }

    let filename = &args[2];
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: File {} not found", filename);
            exit(1);
        }
    };

    let filecontent = fs::read_to_string(filename).unwrap();

    if flag.unwrap() == 'c' {
        let bytes = get_bytes(&filecontent);
        println!("{} {}", filename, bytes);
    } else if flag.unwrap() == 'l' {
        let lines = get_lines(&filecontent);
        println!("{} {}", filename, lines);
    } else if flag.unwrap() == 'w' {
        let words = get_words(&filecontent);
        println!("{} {}", filename, words);
    } else if flag.unwrap() == 'm' {
        let chars = get_chars(filename);
        println!("{} {}", filename, chars);
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
    let flag = arg.chars().nth(1).unwrap();
    match flag {
        'c' => return Ok('c'),
        'l' => return Ok('l'),
        'w' => return Ok('w'),
        'm' => return Ok('m'),
        _ => {
            println!("Usage: rust-wc -[c|l|w|m] <filename>");
            return Err(false);
        }
    }
}

fn is_flag(arg: String) -> bool {
    if arg.chars().nth(0).unwrap() == '-' {
        return true;
    }

    return false;
}