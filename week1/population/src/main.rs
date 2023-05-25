use cs50::get_i32;

fn main() {
    let starting_pop = get_user_input(9, "Start size: ");
    let ending_pop = get_user_input(*&starting_pop, "End size: ");
    let years = calculate_years(&starting_pop, &ending_pop);
    println!("Years: {years}");
}

fn get_user_input(minimum: i32, text: &str) -> i32 {
    let mut input;
    loop {
        input = match get_i32(text) {
            Ok(s) => s,
            Err(_) => continue    
        };
        if input >= minimum {
            break;
        }
    }
    return input;
}

fn calculate_years(start: & i32, end: & i32) -> i32 {
    let mut years = 0;
    let mut current_pop = *start;
    if start == end {
        return 0;
    }
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