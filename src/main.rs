#![allow(unused)]
use std::io::{self, Write};

/* TODO
* Make printing more pretty.
* Read words from a file.
* MAKE FUCKING BACKSPACE WORK...
*/


const MAX_CHAR: usize = 5;

// 0 means no match
// -1 means there is match but in another place
// 1 means correct
fn guess_info(goal_word: &[u8], guess: &[u8]) -> [i8; MAX_CHAR]{
    let mut retarr: [i8; 5] = [0; 5];

    for i in 0..5{
        if guess[i] == goal_word[i] {
			retarr[i] = 1;
		}
    }

    for i in 0..5{
        for j in 0..5{
            if guess[i]==goal_word[j] && guess[i]!=guess[j] && retarr[i] != -1 {
                retarr[i] = -1;
            }
        }
    }
    return retarr;
}

fn print_state_of_game(guess: &[u8], arr: [i8; MAX_CHAR]) {
	for i in 0..5 {
		if arr[i] == 0 { print!("_"); }
		else if arr[i] == -1 { print!("{}", guess[i] as char); }
		else if arr[i] == 1 { print!("{}", guess.to_ascii_uppercase()[i] as char); }
	}
}


fn main_game(goal_word: &[u8]){
	let mut tries: u8 = 5;
	while tries != 0 {
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		let guess_byte: &[u8] = guess.as_bytes();

		let arr = guess_info(goal_word, guess_byte);
		
		print_state_of_game(guess_byte, arr);
		println!("\t\t{tries} tries remaining!");

		tries -= 1;
		if arr.iter().min() == Some(&1) {
			println!("Congrats, you won!");
			break;
		}
	}
}

fn main() {
    let mut goal = String::new();
	print!("\x1B[2J\x1B[1;1H");
	println!("Please enter your goal word: ");
    io::stdin()
        .read_line(&mut goal)
        .expect("Failed to read line");
	print!("\x1B[2J\x1B[1;1H");
	println!("Welcome to the game.\n");
    main_game(goal.to_lowercase().as_bytes());
}
