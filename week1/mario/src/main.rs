use cs50::get_i32;

fn main() {
    let pyramid_height = get_input();
//    draw_pyramid(pyramid_height);
}


fn get_input() -> i32 {
    let mut input;
    loop {
        input = match get_i32("Pyramid height: ") {
            Ok(i) => i,
            Err(_) => continue
        };
        if input > 0 && input < 9 {
            break;
        }
    }
    return input;
}

// fn draw_pyramid(input_height) -> () {

//     for i in 1..=input_height {

//     }
// }

/* 
The goals.
Get input, positive int 1 - 8 inclusive
Draw right aligned pyramid with height of input
*/