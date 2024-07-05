fn main() {
 let result = setting_variables(432, 1234);

 println!("{:?}", result);   
 mutation_variables(8495);

 println!("{}", THE_CONVENTION_IS_CAPITAL_LETTERS);
}


// Now we start handling data
// Here are some operations in rust with int data types
fn setting_variables(num1: i64, num2: i64) -> i64 {
    let sum = num1 + num2;

    let multiply = num1 * num2;

    let minus = num1 - num2;

    let divide = num1 / num2;

    let modulus = num1 % num2;

    return sum + multiply + minus + divide + modulus; 
}

fn mutation_variables(num1: i64) {
    // mut lets you reassign a value later on in code, this can be used when creating loops as an example
    let mut num2 = 45;
    println!("{}", num2);

    num2 = num1;
    println!("{}", num2);

    // When a variable is created it is immutable which is the value cannot changes by reassigning a value
    // let num3 = 123;
    // num3 = num2;
    // println!("{}", num3);
}

// here is a new key word which we can also use with our variables
// const is almost the same as the single 'let' in terms that it is immutable
const THE_CONVENTION_IS_CAPITAL_LETTERS: &str = "This value is globally accessible in this file";

// however you cannot make a const value mutable
//const mut constant_mutable: i64 = 6542 / 2;

