use rand::Rng;
use std::{cmp::Ordering, io, string};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //arm
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    basics1(); // basics1() also valid
    basics2();
}

fn basics1(){
    let x = 5; // variables are immutable
    let mut y = 10; // making var mutable
    y += 1;
    println!("x = {x} and y + 2 = {}", y+2);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = x + 1; //shadowing first x
    println!("time = {}",THREE_HOURS_IN_SECONDS);
    {
        let x = x * 2; // shadowing in inner scope
        println!("The value of x in inner scorep is : {x}")
    }
    // we can use shadowing to change var type
    /* Datatypes - scalar and compound
    scalar - int, float, bool, char
    handle overflows correctly
    rust uses UTF-8 format for chars
    compound - tuples, arrays
     */
    // rust panics and throws error
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; //destructuruing
    println!("{:?}", tup); // max 12 elements
    println!("{}",tup.0); // format string to print macros is mandatory
    // () - unit type which get's returned by expressions by default
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("{}", a[0]);
    // rust uses snake case with underscores
    let x = plus_one(5);
    println!("The value of x is: {x}");
    if x > 5{
        println!("Higher");
    } else if x < 5 {
        println!("Lower");
    }else{
        println!("Same");
    }
    let number = if true {5} else {6}; // if is expression
    // loops - loop, while, for and are expressiosn
    // rust is expression based language
    let mut counter = 0;
    // loop labels
    let result = 'countingup: loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    for element in a{
        println!("value: {element}")
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }


}

fn basics2(){
    // ownership, no garbage collector needed
    // members dropped when go out of scope as rust calls drop automatically
    // stack and heap
    // string literal aka string and String
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let x = 5;
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1;
    // rust prevents double drop erorr as s1 goes out of scope
    // s1 was moved into s2 not shallow or deep copied
    // use .clone() for deep copy of heap data
    // all stack types implement the Copy trait which make them copy instead ofmoving
    // copy can't be annoted if member already implemented Drop trait
    // function arguments ownership works just like vars
    // ownership also moves to funcs and can move out with return to var
    // tuples can be used to return multiple values
    // or use references and they are guaranteed to point to valid data
    // but not mutable as ownernship is not moved
    let s1 = String::from("hello");
    // reference borrowing
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // * = dereferencing, & = referencing
    let mut s = String::from("hello");
    // mutable reference
    change(&mut s);
    //
    // it prevents data races
    // a member can have 1 mutable ref or multiple unmutable ref
    // ref scope begin with introductin and end at last used even in same scope
    // rust prevents dangling pointers
    let reference_to_nothing = dangle();
    // string slices are ref to part of a string
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // without slicing it's hard to tie index to string in sync
    // with slicing other members can't get mut ref to origiinal var
    // but slicing should happen at valid utf8 boundary
    // &str - string literals are slices

}

fn calculate_length(s: &String) -> usize {
    // s has ptr pointing to s1 in stack
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { //&String
    let s = String::from("hello");
    // invalid as s goes out of scope so ref cant be returned
    //&s //just return s as it moves
    s
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x+1; not valid
    // return x + 1 and return x + 1; valid
    // rust implicitly returns only expression at end
    //so explicitly return if needed to do it before
}

fn access_array(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}