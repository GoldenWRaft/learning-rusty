// Working with arrays are extremely useful

// Let's get started

fn main() {
    implicit_array_function();
    explicit_array_function();
    iterating_through_arrays();
}


// There can be 2 types of arrays
// implicit arrays (length are set at runtime)
// explicit arrays (length are set by the developer)

fn implicit_array_function() {
    let numbers: &[i32] = &[1,2,3,4,5,6];
    println!("{:?}", numbers);
    println!("The first time in the array is: {}", numbers[0]);
}

fn explicit_array_function() {
    let numbers: [i32; 3] = [1,2,3];
    //               ^^^ This is how you mark a length of an array
    println!("{:?}", numbers);
    println!("The first time in the array is: {}", numbers[0]);
}

fn iterating_through_arrays() {
    const LENGTH: usize = 3;
    let numbers: [i32; LENGTH] = [1,2,3];

    for number in numbers {
        println!("{}", number);
    }
}