#![allow(unused)]
use std::io;
use rand::Rng;
use std::io:: {Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::collections::HashMap;
use std::cmp::Ordering;

/**
 *  guessing_game: A simple implementation of guessing game
 */
fn guessing_game() {
 println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Error reading input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Error: Input not a number");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than target!"),
            Ordering::Greater => println!("Greater than target!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}

/**
 * fibonacci: Simple memoized fibonacci function
 * @n: Unsigned integer for the nth element of the fibonacci sequence
 * @memo: Hashmap for the memoization
 */
fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 2 {
        return 1;
    }

    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    } else {
        let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
        memo.insert(n, result);
        return result;
    }
}

fn print_hello(name: &mut String) -> u32{
    let name = String::from("Peter");
    println!("Hello {}", name);
    let e: u32 = 5;
    e
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
// }

fn main() {
    
    // let mut memo: HashMap<u64, u64> = HashMap::new();

    // let mut test_string: String = String::from("John");



    // let x = print_hello(&mut test_string);
    // println!("{}", test_string);
    // println!("Fib = {}",fibonacci(90, &mut memo));
    // guessing_game();
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(width: &u32 , height: &u32 ) -> u32 {
            width * height
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect1.width, &rect1.height)
    );


    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!();
        }
    }

    let m = Message::Move{x: 32, y:40};
    match m {
        Message::Move{x, y} => println!("x = {}, y = {}", x, y),
        _ => println!("Not a move"),
        
    }
    let no: Option<i8>;
    // m.call();

    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    // let third: Option<&i32> = v.get(2);
    println!("third = {:?}", third);


}

