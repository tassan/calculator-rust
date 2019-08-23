fn main() {
    println!("Welcome to The Calculator!");
    println!("Here are some examples of calcs:");
    println!("Sum -> 2 + 2 = {}", sum(2, 2));
    println!("Sub -> 2 - 2 = {}", sub(2, 2));
    println!("Mult -> 2 * 2 = {}", mult(2, 2));
    println!("Div -> 2 / 2 = {}", div(2, 2));
}

fn sum(a: i32, b: i32) -> i32 {
    a+b
}

fn sub(a: i32, b: i32) -> i32 {
    a-b
}

fn mult(a: i32, b: i32) -> i32 {
    let mut times = b;
    let mut result = 0;
    
    while times != 0 {
        result = sum(result, a);
        times -= 1;
    }
    
    result
}

fn div(a: i32, b: i32) -> i32 {
    let mut c = 0;
    let mut result = 0;

    c += 1;
    result = sub(a, mult(b, c));
    
    result
}