#![allow(unused)] //allows you to have a variable without using them in your code
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
    println!("what is your name?");
    let mut name = String::new();
    let greeting = "nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("didn't recieve the write input");
    println!("Hello {},{}", name.trim_end(), greeting);
    math();
}

fn math() {
    let num_1 = 15;
    let num_2 = 5;
    println!("15 + 5 = {}", num_1 + num_2);
    println!("15 - 5 = {}", num_1 - num_2);
    println!("15 * 5 = {}", num_1 * num_2);
    println!("15 / 5 = {}", num_1 / num_2);
    println!("15 % 5 = {}", num_1 % num_2);
}
