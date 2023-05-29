use cs50::get_i32;

fn main() {
    let mut cents = get_cents();
    let quarters = calculate_quarters(&cents);
    cents = cents - quarters * 25;
    let dimes = calculate_dimes(&cents);
    cents = cents - dimes * 10;
    let nickels = calculate_nickels(&cents);
    cents = cents - nickels * 5;
    let pennies = calculate_pennies(&cents);
    let coins = quarters + dimes + nickels + pennies;
    println!("{}", coins);
}

fn get_cents() -> i32 {
    let mut cents:i32;
    loop {
        cents = match get_i32("Change owed: ") {
            Ok(s) => s,
            Err(_) => continue
        };
        if cents >= 0 {
            break;
        }
    }
    return cents;
}

fn calculate_quarters(cents: &i32) -> i32 {
    let quarters = cents / 25;
    return quarters;
}

fn calculate_dimes(cents: &i32) -> i32 {
    let dimes = cents / 10;
    return dimes;    
}

fn calculate_nickels(cents: &i32) -> i32 {
    let nickels = cents / 5;
    return nickels;    
}

fn calculate_pennies(cents: &i32) -> i32 {
    let pennies = cents.clone();
    return pennies;    
}