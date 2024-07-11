fn main() {
 immutable_ref();
 mutable_ref();
}

// immutable ref
// a lot of references can be made to an immutable value
fn immutable_ref() {
    let x: i32 = 234;
    let r = &x;
    let z = &x;

    println!("Referenced type: {}", x);
    println!("Referenced type: {}", &r);
    println!("Referenced type: {}", &z);
}

// mutable ref
// 1 mutable reference can be active at a time
fn mutable_ref() {
    let mut x: i32 = 123;
    let y = &mut x;

    // pointing to a referenced mutable
    *y += 32;

    println!("{}", &x);
}