// use std::io;


fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

    println!("The value of s3 is: {}", s3);
}