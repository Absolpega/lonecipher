use clap::Parser;
//use is_vowel::IsRomanceVowel;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(allow_negative_numbers = true)]
    count: isize,

    string: String,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let turn_count: usize = args.count.unsigned_abs();

    let string = args.string;

    let filters = vec![
        (|x| {
            "aeiou"
                .chars()
                .any(|y| y == x || y.to_ascii_uppercase() == x)
        }),
        (|x| {
            "bcdfghjklmnpqrstvwxyz"
                .chars()
                .any(|y| y == x || y.to_ascii_uppercase() == x)
        }),
    ];

    let capitalization_map: Vec<bool> = string.chars().map(|x| x.is_uppercase()).collect();

    let mut filter_strings: Vec<Vec<(usize, char)>> = vec![];

    filters.iter().for_each(|f| {
        let (indices, mut str): (Vec<usize>, Vec<char>) =
            string.char_indices().filter(|x| (f)(x.1)).unzip();

        let str_len = str.len(); // fuck_this

        if str_len != 0 {
            if args.count.is_positive() {
                str.rotate_right(turn_count % str_len);
            } else {
                str.rotate_left(turn_count % str_len);
            }
        }

        filter_strings.push(indices.into_iter().zip(str).collect());
    });

    let mut new_string = string.chars().collect::<Vec<char>>();

    filter_strings
        .iter()
        .rev()
        .for_each(|ts| ts.iter().for_each(|s| new_string[s.0] = s.1));

    let new_string: String = new_string
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if capitalization_map[i] {
                x.to_ascii_uppercase()
            } else {
                x.to_ascii_lowercase()
            }
        })
        .collect();

    println!("{}", new_string);
}
