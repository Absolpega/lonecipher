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

struct Type {
    filter_closure: Box<dyn Fn(char) -> bool>,
    rotate: bool,
}

impl Type {
    fn new(rotate: bool, filter_closure: Box<dyn Fn(<std::str::Chars as Iterator>::Item) -> bool>) -> Self {
        return Self { filter_closure, rotate }
    }
}

fn main() {
    let args = Args::parse();

    let turn_count: usize = args.count.unsigned_abs();

    let string = args.string;

    let types = vec!(
        //Type::new(Box::new(|x: char| x.is_romance_vowel())),
        //Type::new(Box::new(|x: char| x.is_alphabetic() && !x.is_romance_vowel())),
        //Type::new(Box::new(|x: char| !x.is_alphabetic())),
        Type::new(true,     Box::new(|x|    "aeiou"                        .chars().any(|y| y == x || y.to_uppercase().next().unwrap() == x))),
        Type::new(true,     Box::new(|x|    "bcdfghjklmnpqrstvwxyz"        .chars().any(|y| y == x || y.to_uppercase().next().unwrap() == x))),
        Type::new(false,    Box::new(|x| !  "abcdefghijklmnopqrstuvwxyz"   .chars().any(|y| y == x || y.to_uppercase().next().unwrap() == x))),
    );  

    let mut types_strings_with_holes: Vec< Vec<Option<char>> > = vec!();


    let capitalization_map: Vec<bool> = string.chars().map(|x| x.is_uppercase()).collect();

    types
        .iter()
        .for_each(|t|
                  types_strings_with_holes
                  .push(string
                        .chars()
                        .map(|c| (t.filter_closure)(c).then_some(c))
                        .collect()
                       )
                 );

    let old_string_with_holes = types_strings_with_holes.clone();

    types_strings_with_holes.iter_mut().zip(types).for_each(|str| {
        if !str.1.rotate { return; }
        let (indexes, mut tight_string): (Vec<usize>, Vec<Option<char>>) = str.0
            .iter()
            .enumerate()
            .filter(|x| x.1.is_some())
            .unzip();

        if tight_string.len() != 0 {
            let turn_len = turn_count % tight_string.len();

            if args.count.is_positive() {
                tight_string.rotate_right(turn_len);
            } else {
                tight_string.rotate_left(turn_len);
            }
        }

        indexes
            .into_iter()
            .zip(&tight_string)
            .for_each(|(i, v)| str.0[i] = *v);
    });

    //let mut new_string = vec![None::<char>; string.len()];
    //types_strings_with_holes.iter().for_each(|str| {
    //    let mut str_iter = str.iter();
    //
    //    let new_string_2 = new_string.clone();
    //
    //    let mut new_string_iter = new_string_2.iter();
    //
    //    new_string_2.iter().enumerate().for_each(|ns| {
    //        let index = ns.0;
    //        let x = new_string_iter.next().unwrap();
    //        let y = str_iter.next().unwrap();
    //
    //        new_string[index] = match (x, y) {
    //            (Some(x),   Some(_y)) => Some(*x),
    //            (Some(x),   None    ) => Some(*x),
    //            (None,      Some(y) ) => Some(*y),
    //            (None,      None    ) => None,
    //        };
    //    });
    //});

    let mut types_iters: Vec<std::slice::Iter<Option<char>>> = types_strings_with_holes.iter().map(|x| x.iter()).collect();
    let mut new_string = string.chars().map(|_| {
        let mut types_next = vec!();
        for iter in &mut types_iters {
            types_next.push(iter.next().unwrap());
        }
        return types_next
            .iter()
            .filter(|x| x.is_some())
            .collect::<Vec<_>>()[0].unwrap()
    }).collect::<String>();

    new_string = new_string.chars().enumerate().map(|(i, x)| if capitalization_map[i] { x.to_uppercase().next().unwrap() } else { x.to_lowercase().next().unwrap() } ).collect();


    if args.verbose {
        old_string_with_holes       .iter().for_each(|x| println!("{}", x.iter().map(|x| if x.unwrap_or('%') == ' ' { '_' } else { x.unwrap_or(' ') } ).collect::<String>()));
        println!();
        types_strings_with_holes    .iter().for_each(|x| println!("{}", x.iter().map(|x| if x.unwrap_or('%') == ' ' { '_' } else { x.unwrap_or(' ') } ).collect::<String>()));
        println!();
    }

    //println!("{}", new_string.iter().map(|x| x.unwrap_or(' ')).collect::<String>());
    println!("{}", new_string);
}
