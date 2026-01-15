// As the name implies this function returns a number from a &str, which is just a
// view into string data owned by something else. Unlike String, which is
fn string_to_number(s: &str) -> i32 {
    let number = s.parse::<i32>().unwrap();
    return number;
}

// Tests