extern crate clap;

use clap::{ App, Arg };

const DEFAULT_MAX_LEN: i32 = 5;
const DEFAULT_CHAR_SET: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];

fn main() {
    let args = App::new("brute")
        .arg(Arg::with_name("maxlen")
            .short("m")
            .long("maxlen")
            .takes_value(true)
            .value_name("len")
        )
        .get_matches();
    
    let max_len = match args.value_of("maxlen").unwrap_or(&DEFAULT_MAX_LEN.to_string()).parse::<i32>() {
        Ok(len) => len,
        _ => DEFAULT_MAX_LEN
    };

    let char_set = DEFAULT_CHAR_SET.to_vec();

    let results = generate_permutations_to(max_len, &char_set);

    println!("results:");
    println!("{:?}", results);
}

fn generate_permutations_to(max_len: i32, char_set: &Vec<char>) -> Option<Vec<String>> {
    // todo: fix overflow issue
    let n = char_set.len().pow(max_len as u32) as i32;

    let mut results: Vec<String> = vec![];

    // we will need to generate n-many permutations
    for i in 0..n {
        let combination = make_permutation(i, char_set)?;
        results.push(combination);
    }

    Some(results)
}

fn make_permutation(i: i32, char_set: &Vec<char>) -> Option<String> {
    let char_set_len = char_set.len() as i32;

    let mut result: Vec<char> = Vec::new();

    let mut current_value = i;
    loop {
        let remainder = current_value % char_set_len;
        current_value = current_value / char_set_len;

        result.insert(0, *char_set.get(remainder as usize)?);

        if current_value == 0 {
            break;
        }
    }

    Some(result.into_iter().collect())
}
