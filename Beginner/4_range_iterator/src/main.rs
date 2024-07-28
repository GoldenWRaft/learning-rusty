// Iterating through a large set of values can be tedious if you have to manually create
// an array of items, here is a way which you can used to create a range to your liking

fn main() {
    // This will create a range of items up to 5 but not including 5
    for x in 0..5 {
        if x == 4 {
            println!("This is the last item");
        }
        else if x > 4 {
            println!("It was not the end of the line");
        }
    }
    
    println!("");

    // To include 5 as well here is what you should do
    for x in 0..=5{
        if x == 4 {
            println!("This is the last item");
        }
        else if x > 4 {
            println!("It was not the end of the line");
        }
    }
}
