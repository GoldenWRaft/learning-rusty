// Strings and string slices

// Handling strings and &str

fn main() {
    string_slices();

    strings();

    string_ref();
}


fn string_slices() {
    let string_slice: &str = "This is a string slice";

    println!("{}", string_slice);

    let str_array: &[&str] = &["Test", "Lion", "Last one"];

    println!("{:?}", str_array);

    // &str (string slice) immutable
    // stored on the stack
    // AKA faster
}

fn strings() {
    let string: String = "This is a string".to_string();

    println!("{}", string);

    // heap allocated
    // mutable
    let mut from_slice: String = String::from("This is created from a string slice");
    from_slice.push_str(", and a cool perk is that I can also grow.");
    println!("{}", from_slice);
}

fn string_ref() {
    let string: String = String::from("This will be copied.");
    let slice: &str = &string[0..7];

    println!("Slice Value: {}", slice);
}