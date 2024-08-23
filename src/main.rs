use std::{env, fs::File, io::{BufRead, BufReader}, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust-wc -[c|l|w|m] <filename>");
        return;
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

    if flag.unwrap() == 'c' {
        let bytes = get_bytes(file);
        println!("{} {}", filename, bytes);
    } else if flag.unwrap() == 'l' {
        let lines = get_lines(file);
        println!("{} {}", filename, lines);
    } else if flag.unwrap() == 'w' {
        let words = get_words(file);
        println!("{} {}", filename, words);
    } else if flag.unwrap() == 'm' {
        // count_chars(file, filename);
    }


}

fn get_bytes(f: File) -> u64 {
    return f.metadata().unwrap().len();
}

fn get_lines(f: File) -> usize {
    return BufReader::new(f).lines().count();
}

fn get_words(f: File) -> usize {
    let words: BufReader<_> = BufReader::new(f);
        
    let mut count = 0;

    for line in words.lines() {
        let line = line.unwrap();
        let words = line.split_whitespace();
        count += words.count();
    }

    return count;
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