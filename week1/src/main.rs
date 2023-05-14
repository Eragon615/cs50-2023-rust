use cs50::get_i32;

fn main() {
    println!("Please enter a starting population of llamas, at least 9.");
    let starting_pop = get_user_input(9);
    println!("Great, now input your target ending population.");
    let ending_pop = get_user_input(*&starting_pop);
    let years = calculate_years(&starting_pop, &ending_pop);
    println!("It will take {years} years to reach the a population of {ending_pop}.");
}

fn get_user_input(minimum: i32) -> i32 {
    let mut input;
    loop {
        input = match get_i32("Please enter an integer: "){
            Ok(s) => s,
            Err(_) => {
                println!("That's not an integer!");
                continue
            }    
        };
        if input > minimum {
            break;
        }
        else {
            println!("Your input must be higher than {minimum}.");
        }
    }
    return input;
}

fn calculate_years(start: & i32, end: & i32) -> i32 {
    let mut years = 0;
    let mut current_pop = *start;
    loop {
        let increase = current_pop / 3;
        let decrease = current_pop / 4;
        current_pop = current_pop + increase - decrease;
        years += 1;
        if current_pop >= *end {
            break
        }
    }
    return years;
}