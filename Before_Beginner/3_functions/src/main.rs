/*
These are the bear basics of functions in rust

Functions are used quite a lot and is probably as essential as the basic data types
*/

fn main() {
    // Here I execute my functions which were defined elsewhere throughout my program.
    // My function executes here
    my_first_function();

    let false_value = return_false();
    let true_value = return_true();

    println!("{:?}", working_string_return());

    working_with_parameter(false, 32);
}

fn my_first_function() {
    println!("This is a function / method which will execute code inside and will return nothing further!");
}

// Let's make it a bit more interesting from here

// Returning values from a function / method
// note that there are 2 ways of returning a value

fn return_false() -> bool {
    false
}

// return values can be basic data types (or complex types but we'll get there)
fn return_true() -> bool {
    // common throughout all other languages
    return true;
}

/*
    Struggling with strings? Yup me too.

    fn string_error() -> str {
        return "There is a whole host of possibilities now open";
    }

    This function has compiler issues
*/

fn working_string_return() -> String {
    "Now we have a standard or step to work from".to_string() // thus far I do not understand this 
}

// when we want values from other functions we can put them in parameters
// you can have as many parameters as you want (but this will become repetitive if you are trying to reuse such function)
// as best practice and ease of use try to keep parameters to a max of 5 I'd say (you can go further of course)
fn working_with_parameter(param1: bool, param2: i32) {
    let storing_param1 = param1;
    let storing_param2 = param2;
}