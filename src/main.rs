use std::io;
use rand::seq::SliceRandom;
use itertools::Itertools;

fn main() {
    println!("");
    println!("--------------------------------------------------------------------------");
    println!("Welcome to 'Burn that Dictator', where the stakes have never been hotter!");
    println!("Guess a letter.");
    println!("--------------------------------------------------------------------------");

    let dictator_list = vec![
        "Adolf Hitler",
        "Bashar al-Assad",
        "Robert Mugabe",
        "Joseph Stalin",
        "Mao Zedong",
        "Idi Amin",
        "Fidel Castro",
        "Kim Jong-un",
        "Saddam Hussein",
        "Vladmir Lenin",
        "Ho Chi Minh",
        "Pol Pot",
        "Kim Il-Sung",
    ];
    let random_name =
        dictator_list
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

    let mut revealed_name: String =
        random_name.chars()
            .map(|l| {
                if l == ' ' {
                    ' '
                } else if l == '-' {
                    '-'
                } else {
                    '*'
                }
            })
            .collect();

    let feedback_list = vec![
        "Honestly, how have you made it this far in life?",
        "We're gonna be here all day.",
        "Please tell me you know what a letter is.",
        "Someone's on the struggle bus today.",
        "This is just painful.",
        "You're hopeless.",
        "I said ONE LETTER, dipstick."
    ];
    
    let mut guesses = Vec::new();

    
    loop {
        let mut guess = String::new();
        let rnd_feedback =
            feedback_list
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        
        let guess: char = match guess.trim().parse() {
            Ok(s) => {
                if guess.chars().nth(0).unwrap().is_alphabetic() {
                    guesses.push(s);
                } else {
                    println!("{}", rnd_feedback)
                }
                s
            },
            Err(_) => {
                println!("Try a single letter next time, idiot.");
                println!("");
                continue
            },
        };
        
        let r = revealed_name.chars()
            .zip(random_name.chars())
            .map(|(revealed_name_char, random_name_char)| {
                if guess.eq_ignore_ascii_case(&random_name_char) {
                    random_name_char
                } else {
                    revealed_name_char
                }
            })
            .collect::<String>();

        revealed_name = r;

        let sep = ',';
        let guesses_str: String = guesses.iter().intersperse(&sep).collect();

        if revealed_name == random_name {
            println!("{} has been successfully eradicated. The world is a better place and all that - Next!", revealed_name);
            println!("Total guesses: {}", guesses.len());
            println!("--------------------------------------------------------------------------");
            println!("");
            break
        } else {
            println!("Dictator: {}", revealed_name);
            println!("Guesses: {}", guesses_str);
            println!("");
        }
    }
}
