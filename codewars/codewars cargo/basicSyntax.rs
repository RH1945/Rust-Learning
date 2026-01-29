fn x_i32() {
    let x::i32 = 6; // var must be declared before using it. i.e. if you call foobar(x) before
                    // this line, it won't find x (borrowing issue)
    foobar(x); // the type of `x` will be inferred from here
}

// The underscore _ is a special name, or rather, a “lack of name”.
// It basically means to throw away something:
fn discarding_values() {
    let x::i32 = 12;
    let _ = get_thing();
    return x;
}

//Separate bindings with the same name can be introduced - you can shadow a variable binding:
fn shadowing_bindings() {
    let x::i32 = 12;
    let x = x + 6;
    return x;
}

//Rust has tuples, which you can think of as “fixed-length collections of values of different types”.
fn tuples() {
    let pair = ('x', 24);
    pair.0; // 'x'
    pair.1; // 24
            //We can also annotate the type of pair:
    let pair2: (char, i32) = ('y', 32);
}

//Tuples can be destructured when doing an assignment, which means
// they’re broken down into their individual fields:
fn destructing_tuples() {
    let destruct_tuple: (some_char, some_int) = ('z', 40);
    // now, `some_char` is 'z', and `some_int` is 40
    let (left, right) = slice.split_at(middle);
    return destruct_tuple;
    let (_, right) = slice.split_at(middle);
    return destruct_tuple;
}

//Statement go until you hit a semicolon
fn statements() {
    let x = 6;
    let y = 12;
    let z = y + x;
    //This means we can go for multiple lines:
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    return x;
}

// fn declares a function:
fn hello() {
    println!("Hello universe!");
    //will print Hello universe!
}

//What we are doing here is returning a 32-bit signed integer.
// The arrow indicates its return type:
fn dice_roll() -> i32 {
    6
}

//Now blocks:
fn main() {
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}

// this:
//let x = 42;

// is equivalent to this:
// let x = { 42 };
// Inside a block, there can be multiple statements:
//     let x = {
//     let y = 1; // first statement
//     let z = 2; // second statement
//     y + z // this is the *tail* - what the whole block will evaluate to
// };
//^
//why “omitting the semicolon at the end of a function” is the same as returning, i.e. these are equivalent:
// implicit_return
fn fair_dice_roll() -> i32 {
    return 4;
}
fn fair_dice_roll() -> i32 {
    4
}

// Everything is an expression

//if conditionals are also expressions:
fn fair_dice_roll() -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}
// match is also an expression:
fn fair_dice_roll() -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}