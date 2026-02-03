fn fizz_buzz(n: i32) -> vec<String> {
    (1..=100)
        .map(|x| match x {
            x if x % 15 == 0 => String::from("FizzBuzz"),
            x if x % 3 == 0 => String::from("Fizz"),
            x if x % 5 == 0 => String::from("Buzz"),
            _ => x.to_string(),
        })
        .collect()
}
