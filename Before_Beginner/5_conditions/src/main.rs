// Conditions in programming is an essential part of logical executions 
// writing pseudo (reading it like sue dough) code will be something like the following

/*
if my_number is value
    then execute the following
else if my_number is value
    then execute this
else 
    do this
*/

fn main() {
    condition_handling_with_bug(32);

    condition_handling_without_bug(22);
}

// Notice that my code is not executing correctly
// 18 is still > 18 but a bug occurred (not expected behavior)
fn condition_handling_with_bug(age: i32) {
    if age > 13 {
        println!("Teenage years!");
    }
    else if age > 18 {
        println!("Young adult!")
    }
    else {
        println!("Adult!");
    }
}

// If an expected result is not reached the first time around 
// just try again.
fn condition_handling_without_bug(age: i32) {
    if age > 25 {
        println!("Adult!");
    }
    else if age > 18 {
        println!("Young adult!")
    }
    else if age > 13 {
        println!("Teenage years!");
    }
    else {
        println!("Very young!")
    }
}