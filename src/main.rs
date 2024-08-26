use std::io;

const MAX_CHAR: usize = 5;

// 0 means no match
// -1 means there is match but in another place
// 1 means correct
fn guess_info(goal_word: &String, guess: &String) -> [i8; MAX_CHAR]{
    let mut retarr: [i8; 5] = [0; 5];
    let mut check_arr: [i8; 5] = [0; 5];
    let b_goal = goal_word.as_bytes();
    let b_guess = guess.as_bytes();

    for i in 0..5{
        if b_guess[i] == b_goal[i] {retarr[i] = 1;}
    }

    for i in 0..5{
        for j in 0..5{
            if b_guess[i]==b_goal[j] && b_guess[i]!=b_guess[j] && check_arr[j]==0 {
                retarr[i] = -1;
                check_arr[j] = 1;
            }
        }
    }
    return retarr;
}


fn main_game(goal_word: &String){
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess = guess.to_lowercase();
    let arr = guess_info(goal_word, &guess);
    println!("{arr:?}");
}

fn main() {
    let mut goal = String::new();
    io::stdin()
        .read_line(&mut goal)
        .expect("Failed to read line");
    let b = goal.to_lowercase().as_bytes();
    main_game(&goal);
}
