extern crate clap;

use std::fs::File;
use std::io::Read;
use clap::{App, Arg};

const DEFAULT_MAX_LEN: i32 = 5;

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

    let char_set_file = args.value_of("charset").unwrap_or("./resources/charset");
    let char_set = Ok(File::open(char_set_file)
            .unwrap_or(File::open("./resources/charset").unwrap()))
        .and_then(|mut file| {
            let mut result = String::new();

            file.read_to_string(&mut result)
                .and_then(|size| {
                    let str_iter: Vec<char> = result.split(',')
                        .into_iter()
                        .map(|s: &str| {
                            let index: usize = 0 as usize;
                            let c: Vec<char> = s.chars().collect();

                            *c.get(index).unwrap()
                        })
                        .collect();

                    Ok(str_iter)
                })
        })
        .unwrap();
    
    println!("char_set: {:?}", &char_set);

    let max_num_results = char_set.len().pow(max_len as u32) as i64;
    let results = generate_permutations(0, 2000, &char_set);

    println!("results:");
    println!("{:?}", results);
}

fn generate_permutations(start: i64, end: i64, char_set: &Vec<char>) -> Option<Vec<String>> {
    let mut results: Vec<String> = vec![];

    // we will need to generate n-many permutations
    for i in start..end {
        let combination = make_permutation(i, char_set)?;
        results.push(combination);
    }

    Some(results)
}

fn make_permutation(i: i64, char_set: &Vec<char>) -> Option<String> {
    let char_set_len = char_set.len() as i32;

    let mut result: Vec<char> = Vec::new();

    let mut current_value = i;
    loop {
        let remainder = current_value % char_set_len as i64;
        current_value = current_value / char_set_len as i64;

        result.insert(0, *char_set.get(remainder as usize)?);

        if current_value == 0 {
            break;
        }
    }

    Some(result.into_iter().collect())
}
