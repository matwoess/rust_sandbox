use rand::prelude::IteratorRandom;
use crate::util;

pub(crate) fn main() {
    println!("> starting \"guess\" module");
    print!("Number in range from 0 to?: ");
    let until = util::prompt_for_usize();
    if until == usize::MAX {
        return;
    }
    print!("Number of tries?: ");
    let max_tries = util::prompt_for_usize();
    if max_tries == usize::MAX {
        return;
    }

    let mut rng = rand::thread_rng();
    let nr = (0..=until).into_iter().choose(&mut rng).unwrap();
    println!("Guess which number between 0 and {} was chosen!", until);
    let mut n_guesses = 0;
    loop {
        print!("Guess: ");
        let guess = util::prompt_for_usize();
        if guess == usize::MAX {
            println!("try again!");
            continue;
        }
        n_guesses = n_guesses + 1;
        if guess < nr {
            println!("Too low!");
        } else if guess > nr {
            println!("Too high!");
        } else {
            println!("That's it, you won! Good guess!");
            break;
        }
        if n_guesses >= max_tries {
            println!("Sorry, you reached the maximum number of {} guesses. You lose!", max_tries);
            break;
        }
    }
}