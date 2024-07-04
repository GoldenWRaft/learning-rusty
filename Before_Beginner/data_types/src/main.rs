// these are the absolute basic data types which are virtually in all languages

fn main() {
    let is_boolean: bool = true;

    // the following ints have these max values positive and negative
    let byte_sized_int: i8 = 127; 
    let small_int: i16 = 32767;
    let standard_int: i32 = 2147483647;
    let long_int: i64 = 9223372036854775807;
    let extreme_int: i128 = 170141183460469231731687303715884105727;

    // the following are unsigned ints which do not use negative values
    let byte_sized_uint: u8 = 255;
    let small_uint: u16 = 65535;
    let standard_uint: u32 = 4294967295;
    let long_uint: u64 = 18446744073709551615;
    let extreme_uint: u128 = 340282366920938463463374607431768211455;

    // floats: working with decimal numbers
    let standard_float: f32 = 15649879654654984654321321684985465.5465489654648465432168487;
    let large_float: f64 = 156498795654646849846543128465421312466546545.5465486546543213216496874974987;

    // characters / chars
    // note that this is using the ' single quote
    let simple_char: char = 'c';
    let why_not_emojies: char = '👌'; // for some reason this works

    // strings
    let simple_string: str = "This is a simple example of strings!";
}
