fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if number < 5 { 5 } else { 6 };
    println!("The value of number is: {number}");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    add_str(&mut s2);
    println!("s2 is: {s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_str(s: &mut String) {
    s.push_str(" world");
}
