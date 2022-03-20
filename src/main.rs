use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use termion::color;
use termion::style;

static WORDS: [&str; 11] = [
    "fish",
    "rust",
    "notebook",
    "tree",
    "bottle",
    "earth",
    "butter",
    "shoes",
    "fortnite",
    "earphones",
    "jacket",
];

fn main() {
    let mut rng = thread_rng();

    loop {
        let word = WORDS
            .choose(&mut rng)
            .expect("Word List is empty. Aborting!");

        let word = (*word).clone();
        let mut lives = 5;

        let mut used_words = vec![];

        let mut answer = vec!["_".to_string(); word.len()];

        print!("\x1B[2J\x1B[1;1H");

        while lives > 0 {
            print_lives(lives);
            println!("{}", answer.join(" "));
            println!("Take a guess");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read input");

            guess = guess.trim().to_lowercase().to_string();

            print!("\x1B[2J\x1B[1;1H");

            /*
                Validation
                1. Input should have a single character
                2. Should not be already used letter
            */
            if guess.len() != 1 {
                print_error("Invalid input");
                continue;
            }

            let guess = guess.chars().next().unwrap();

            if used_words.contains(&guess) {
                print_error("This letter is already used.");
                continue;
            } else {
                used_words.push(guess);
            }

            let mut idx = 0;
            let mut is_correct_guess = false;

            for c in word.chars() {
                if c == guess {
                    answer[idx] = guess.to_string();
                    is_correct_guess = true;
                }
                idx += 1;
            }

            if !answer.contains(&"_".to_string()) {
                print_success("You Won! 🎉");
                break;
            }

            if !is_correct_guess {
                lives -= 1;
            }
        }

        if lives == 0 {
            print_error("You have used all your lives 😕");
            println!(
                "The word was {}{}{}{}{}",
                style::Bold,
                color::Fg(color::Green),
                word,
                color::Fg(color::Reset),
                style::Reset
            );
        }

        if !should_play_again() {
            break;
        }
    }
}

fn print_success(msg: &str) {
    println!(
        "{}{}{}",
        color::Fg(color::Green),
        msg,
        color::Fg(color::Reset)
    );
}

fn print_error(msg: &str) {
    println!(
        "{}{}{}",
        color::Fg(color::Red),
        msg,
        color::Fg(color::Reset)
    );
}

fn print_lives(lives: i32) {
    for _ in 0..lives {
        print!("🧡");
    }
    println!("");
}

fn should_play_again() -> bool {
    loop {
        println!("Play Again y/n?");
        let mut res = String::new();
        io::stdin()
            .read_line(&mut res)
            .expect("Failed to read input");

        let res = res.trim();

        if "yYnN".contains(res) {
            if res == "y" || res == "Y" {
                return true;
            } else if res == "n" || res == "N" {
                return false;
            }
        }

        println!("Please enter either y or n");
    }
}
