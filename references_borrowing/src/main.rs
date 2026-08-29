fn main() {
    let message = String::from("hello");
    let length = calculate_length(&message);

    // `message` is still owned here because calculate_length only borrowed it.
    println!("The length of '{message}' is {length}.");

    let mut greeting = String::from("hello");
    add_world(&mut greeting);
    println!("A mutable borrow can change the value: {greeting}");

    // Multiple immutable references can exist at the same time.
    let first_reader = &greeting;
    let second_reader = &greeting;
    println!("Immutable references: {first_reader} and {second_reader}");

    // The immutable references are no longer used, so a mutable borrow is valid.
    add_exclamation(&mut greeting);
    println!("After the later mutable borrow: {greeting}");
}

fn calculate_length(value: &String) -> usize {
    value.len()
}

fn add_world(value: &mut String) {
    value.push_str(", world");
}

fn add_exclamation(value: &mut String) {
    value.push('!');
}
