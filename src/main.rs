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
    let mut iterator: usize = 0;

    for (goal_indice, guess_indice) in b_goal.iter().zip(b_guess.iter()){
        if goal_indice==guess_indice{
            retarr[iterator] = 1;
            check_arr[iterator] = 1;
        }
        iterator += 1;
    }
    
    iterator = 0;
    for goal_indice in b_goal.iter(){
        for guess_indice in b_guess.iter(){
            
        }
        iterator += 1;
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
    let goal = String::from("kalem");
    main_game(&goal.to_lowercase());
    
    println!("Hello, world!");
}
