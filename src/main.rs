extern crate clap;

use std::fs::File;
use std::io::Read;
use clap::{App, Arg};

const DEFAULT_MAX_LEN: i32 = 5;
const DEFAULT_CHARSET_FILE: &'static str = "./resources/charset";

fn main() {
    let args = App::new("brute")
        .arg(Arg::with_name("maxlen")
            .short("m")
            .long("maxlen")
            .takes_value(true)
            .value_name("len"))
        .arg(Arg::with_name("charset")
            .short("c")
            .long("charset")
            .takes_value(true)
            .value_name("file"))
        .get_matches();

    let max_len =
        match args.value_of("maxlen").unwrap_or(&DEFAULT_MAX_LEN.to_string()).parse::<i32>() {
            Ok(len) => len,
            _ => DEFAULT_MAX_LEN,
        };

    let charset_file = args.value_of("charset").unwrap_or(DEFAULT_CHARSET_FILE);
    let charset = Ok(File::open(charset_file)
            .unwrap_or(File::open(DEFAULT_CHARSET_FILE).unwrap()))
        .and_then(|mut file| {
            let mut result = String::new();

            // read the file into our String
            file.read_to_string(&mut result)
                .and_then(|_| {
                    // convert the contents of the file into a vec
                    let chars: Vec<char> = result.chars().collect();

                    Ok(chars)
                })
        })
        .unwrap();
    
    println!("char_set: {:?}", &charset);

    // we will need to generate at most radix^max_len where char_set_len is the radix
    let max_num_results = charset.len().pow(max_len as u32) as i64;
    let results = generate_permutations(0, 2000, &charset);

    println!("results:");
    println!("{:?}", results);
}

fn generate_permutations(start: i64, end: i64, charset: &Vec<char>) -> Option<Vec<String>> {
    let mut results: Vec<String> = vec![];

    for i in start..end {
        let combination = make_permutation(i, charset)?;
        results.push(combination);
    }

    Some(results)
}

fn make_permutation(i: i64, charset: &Vec<char>) -> Option<String> {
    let char_set_len = charset.len() as i32;

    let mut result: Vec<char> = Vec::new();

    let mut current_value = i;
    loop {
        let remainder = current_value % char_set_len as i64;
        current_value = current_value / char_set_len as i64;

        result.insert(0, *charset.get(remainder as usize)?);

        if current_value == 0 {
            break;
        }
    }

    Some(result.into_iter().collect())
}
