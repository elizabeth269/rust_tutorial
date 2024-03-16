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
    random_num();
    age();
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

fn random_num() {
    let random_num_ = rand::thread_rng().gen_range(1..=100);
    println!("random: {}", random_num_);
    println!("random: {random_num_}")
}
//dont eat me alive, im just learning.
//besides, rust can't be the furst language you learn in my opinion
fn age() {
    println!("what is your Age?");
    let mut u_age = String::new();
    io::stdin()
        .read_line(&mut u_age)
        .expect("your age is not a number");
    if (u_age >= 1.to_string()) && (u_age <= 17.to_string()) {
        println!("you are a minor");
    } else if (u_age > 17.to_string()) && (u_age < 35.to_string()) {
        println!("you are a young Adult")
    } else if u_age > 34.to_string() {
        println!("you are old")
    } else {
        println!("bro your age doen't exist")
    }
}

fn using_match() {
    let age = 9;
    match age {
        1..=18 => println!("you are important"),
        65..=i32::MAX => println!("you are valuabel"),
    }
}
