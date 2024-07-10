// Understanding loops can sometimes be frustrating
// Here are some simple examples which the foundations can be built upon

fn main() {
    basic_loop();

    let result = loop_result();

    println!("{:?}", result);

    while_loop();

    for_loop();
}

// With this loop we can easily create a infinite running loop which can be great
// for using in games and various other places
fn basic_loop() {
    let mut iterations = 0;

    // this loop will run infinitely if no break is used
    loop {
        if iterations == 10 {
            break
        }

        iterations += 1;

        println!("{:?}", iterations);
    }

    println!("Finished running loop")
}

// Here we can return the result from the loop at any point 
fn loop_result() -> i32 {
    let mut iterations: i32 = 0;

    let result = loop {
        iterations += 1;

        if iterations == 60 {
            break iterations // notice that the result will be after a break
        }
    };

    result
}

// while loops are extremely useful because of the conditional evaluations
fn while_loop() {
    // conditional loop

    let mut running = true;
    let mut number = 2;

    while running {
        number -= 1;

        if number == -50 {
            println!("Stopping now");
            running = false;
        }
    }
}

// for loops are probably the most used loops
// mostly they are used to iterate through copious amounts of data
fn for_loop() {
    let a = [1,2,3,4,5,6,7,8,9];

    for number in a {
        println!("{}", number);
    }
}