use std::io;
use rand::Rng;
use std::time::Duration;
use std::thread;
use std::collections::HashSet;

fn draw_stick_man(incorrect: usize, winner: bool) {
    match incorrect {
        0 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |        ");
            println!(" |        ");
            println!(" |        ");
            println!("_|________");
        }
        1 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |        ");
            println!(" |        ");
            println!("_|________");
        }
        2 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |     |  ");
            println!(" |        ");
            println!("_|________");
        }
        3 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |    /|  ");
            println!(" |        ");
            println!("_|________");
        }
        4 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |    /|\\ ");
            println!(" |        ");
            println!("_|________");
        }
        5 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |    /|\\ ");
            println!(" |    /   ");
            println!("_|________");
        }
        6 => {
            println!("  _____   ");
            println!(" |     |  ");
            println!(" |     O  ");
            println!(" |    /|\\ ");
            println!(" |    / \\ ");
            println!("_|________");
        }
        _ => {
            println!("Invalid value for incorrect.")
        }
    }

    if winner {
        let duration = Duration::from_secs_f64(0.5);
        for _ in 0..3 {
            println!("          ");
            println!("          ");
            println!("          ");
            println!("       O  ");
            println!("      \\|/ ");
            println!("______/_\\_____");
            thread::sleep(duration);

            println!("          ");
            println!("          ");
            println!("          ");
            println!("       O  ");
            println!("      /|/ ");
            println!("______/_\\_____");
            thread::sleep(duration);

            println!("          ");
            println!("          ");
            println!("          ");
            println!("       O  ");
            println!("      /|\\ ");
            println!("______/_\\_____");
            thread::sleep(duration);
        }
    }
}

fn main() {
    println!();
    println!();
    println!();
    println!();

    // Randomly select a word
    let ans_bank = vec!["Nephi", "Lehi", "Leah", "Alma", "Helman", "Mosiah", "Lisa", "Hannah", "Benjamin", "Ehter", "Jarden", "Ammon", "Abinadi", "Zarahmemla", "Enos", "Jarom", "Omin", "Mormon"];
    
    let rand_int = rand::thread_rng().gen_range(0..ans_bank.len());
    let code_word = ans_bank[rand_int].to_uppercase();

    let mut letter_set: HashSet<char> = HashSet::new();
    for letter in code_word.chars() {
        letter_set.insert(letter);
    }

    let mut correct_guesses_set: HashSet<char> = HashSet::new();
    let mut incorrect_guesses: Vec<char> = Vec::new();
    let mut solved = false;
    let duration = Duration::from_secs(2);

    println!("Guess the word to save the man!");

    while !solved && incorrect_guesses.len() < 6 {
        draw_stick_man(incorrect_guesses.len(), solved);

        // Print the current state of the word
        for letter in code_word.chars() {
            if correct_guesses_set.contains(&letter) {
                print!("{letter} ");
            } else {
                print!("_ ");
            }
        }
        println!();
        println!();

        // Print previous guesses
        println!("Guessed Letters: {:?}", incorrect_guesses);
        
        println!("Guess a letter!");
        let mut guess_letter = String::new();
        io::stdin()
            .read_line(&mut guess_letter)
            .expect("Failed to read line");
        
        let guess_letter = guess_letter.trim().to_uppercase();

        if guess_letter.len() == 1 {
            let letter = guess_letter.chars().next().unwrap();

            if !incorrect_guesses.contains(&letter) && !correct_guesses_set.contains(&letter) {
                if code_word.contains(letter) {
                    correct_guesses_set.insert(letter);
                    println!("Great job! {letter} is in the word!!");
                } else {
                    incorrect_guesses.push(letter);
                    println!("{letter} is not in the word!");
                }

                if letter_set == correct_guesses_set {
                    solved = true;
                }
            } else {
                println!("You already guessed that letter!");
            }
        } else {
            println!("Please enter exactly one letter");
        }

        thread::sleep(duration);
    }

    draw_stick_man(incorrect_guesses.len(), solved);

    if solved {
        println!("You Won!!! He survived!! The word was {code_word}!");
    } else {
        println!("You lost! The word was {code_word}");
    }
}
