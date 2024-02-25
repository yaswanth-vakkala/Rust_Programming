use rand::Rng;
use std::{cmp::Ordering, io, string};
// many libraries get imported by defualt with prelude
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
    basics3();
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

fn basics3() {
    struct User {
        active : bool,
        username: String,
        email: String,
        sign_in_count : u64
    }
    // &str wont work but only String refer to lifetimes later
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username,
    //         email,
    //         sign_in_count: 1,
    //     }
    // }
    // user 1 will be no longer valid, but if email, username given new strings then user 1 valid
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0,0,0);
    // unit like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    /*  ex of struct - take rect class with area func both width, height are unrelated
    we can take a struct but syntax of .0, .1 is weird and expressive
    using structs we can convey meanning and have them related
    */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // struct wont implement Display, but can Debug trait
    println!("rect1 is {:?}", rect1); // {:#?} also valid
    // this takes ownership of expression and prints to stderr not stdout
    dbg!(&rect1);
    // rust automatically referencing and dereferencing
    // p1.distance(&p2) = (&p1).distance(&p2)
    // :: is valid for namespaces created by modules and associated funcs 
    let sq = Rectangle::square(3);
    // instead of using enum inside struct we can do this
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // or put structs inside enum as rust std does
    // any enum type can be passed into the below func
    // fn route(ip_kind: IpAddrKind) {}
    // route(IPAddrKind::V4);
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    // rust doesnt have null but Option enum
    // we seen Result enum which handles errors
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    // it handles none as we are required to explicitly handle it and to use it convert it into T
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x { // matches are exhaustive so all cases need to be handled in enum
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other),
    //     _ => () //  _ = this same as above but doen't bind to value so unused var warning
    // }
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // to
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    else{
        // else can also be added which again equals to above match
    }
    /* 
    A package can contain multiple binary crates and optionally 1 library crate
    rust uses module system which has tools to organize code
    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module
    crate root file, package must contain atlease 1 create binary or library
    cargo is a package with multiple creates - main.res, lib.rs 
    or multiple binary crates in src/bin folder
    compiler looks for modules, submodules in inline, src/name.rs, src/name/mod.rs
    submodules = create:garden:vegetables.rs where garden is parent mod with src/garden.rs file
    Path: create::garden::vegetables::Tomato
    modules are private by default, to make public use pub mod, pub
    use creates shortcuts within a scope
    use create::garden::vegetables::Tomato now Tomato can be directly used
    use crate::garden::vegetables::Asparagus;

    pub mod garden;
    
    fn main() {
        let plant = Asparagus {};
        println!("I'm growing {:?}!", plant);
    }
    cargo new restuarent --lib // library create
    cargo new restuarent // binary create
    contents of main.res and lib.rs form create roots so mod starts with create
    for external create Path starts with its name
    absolute and relative path i.e, self, super
    parent mod cant use child private items but child can use parent items
    make mod pub and also its items pub to be accessed by others
    generally write sharable code in library crates
    structs are private and all its members too and to make them public struct and each items should be made pub
    making enums public makes all its members public
    use keyword to bring modules into scope
    mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
    move the use into customer mod  or use super to fix it
    Idomatic use Paths
    as for aliasing
    use std::io::Result as IoResult;
    reexporting using pub use
    crate or library name at start of absolute path
    mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
    now restaurent::hosting::add_to_waithlist() is also valid
    nested paths use std::{cmp::Ordering, io} instead of  2 use lines
    use std::io::{self, Write};
    glob operator * 
    use std::collections::*; bring all public items in
    splitting mods into multiple files
    */
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// multiple impl blocs are allowed
impl Rectangle {
    // self : &Self = &Rectangle - self shorthand below
    // can have same name as fields
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // assosiated funcs which dont have self as 1st parameter used for constructors usually
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
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