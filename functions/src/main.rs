fn main() {
    println!("Hello, world!");
    new_function(40);
}

fn new_function(n: i32) {
    println!("New function with parameter: {n}");
}

fn function_with_return() -> i32 {
    5
}

fn function_with_parameters(x: i32, y: i32) -> i32 {
    return x + y;
}
