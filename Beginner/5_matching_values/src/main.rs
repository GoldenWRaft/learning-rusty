// Writing if statements can get repetitive to write
// and here we'll see how to use the match clause in rust

fn main() {
    let x = 33;

    match x {
        0 => {
            println!("Zero found");
        }

        // 1 or 2 have been found
        1 | 2 => {
            println!("One or two found, can't tell");
        }

        3..=9 => {
            println!("Value between three and nine found");
        }

        matched_num @ 10..=100 => {
            println!("Value between ten and one hundred found: {}", matched_num);
        }

        _ => {
            println!("Default value here");
        }
    };
}
