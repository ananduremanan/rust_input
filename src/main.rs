mod input;

fn main() {
    let name: String = input::input()
        .message("Enter your name: ")
        .get_input()
        .expect("Not a valid input");
    println!("Hello, {}!", name);

    let age: i32 = input::input()
        .message("Enter your age: ")
        .get_input()
        .expect("Not a valid input");
    println!("You are {} years old.", age);

    let height: f64 = input::input()
        .message("Enter your height in meters: ")
        .get_input()
        .expect("Not a valid input");
    println!("You are {} meters tall.", height);
}
