fn main() {
    println!("Welcome to The Calculator!");
    println!("Here are some examples of calcs:");
    println!("Sum -> 2 + 2 = {}", sum(2, 2));
    println!("Sub -> 2 - 2 = {}", sub(2, 2));
    println!("Mult -> 2 * 2 = {}", 4);
    println!("Div -> 2 / 2 = {}", 1);
}

fn sum(a: i32, b: i32) -> i32 {
    a+b
}

fn sub(a: i32, b: i32) -> i32 {
    a-b
}
