use cs50::get_i32;

fn main() {
    let pyramid_height = get_input();
    draw_pyramid(pyramid_height);
}

fn get_input() -> i32 {
    let mut input;
    loop {
        input = match get_i32("Pyramid height: ") {
            Ok(i) => i,
            Err(_) => continue,
        };
        if input > 0 && input < 9 {
            break;
        }
    }
    return input;
}

fn draw_pyramid(input_height: i32) -> () {
    for i in 1..=input_height {
        let spaces = &input_height - i;
        print_chars(spaces, ' ');
        print_chars(i, '#');
        print!("\n");
    }
}

fn print_chars(number_of_times: i32, char_to_print: char) -> () {
    let mut cycle = 0;
    loop {
        if cycle == number_of_times {
            break;
        }
        print!("{}", char_to_print);
        cycle += 1;
    }
}
