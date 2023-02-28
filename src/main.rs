use regex::Regex;
use std::io::stdin;

fn main() {
    // We use regular expressions
    let re_multiplication = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_division = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_addition = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_subtraction = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    // We get user data
    println!("Please, introduce your expression to calculate: ");
    let mut expression: String = String::new();
    stdin().read_line(&mut expression).unwrap();

    // Multiplication
    loop {
        // We use user data and generate a result
        let caps = re_multiplication.captures(&expression);

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let multiplication: i32 = left_value * right_value;
        expression = expression.replace(cap_expression, &multiplication.to_string());

        /* For generate more details about its output we can use:
        println!("{:?} left: {}, right: {}.", caps, left_value, right_value); */
    };

    // Division
    loop {
        // We use user data and generate a result
        let caps = re_division.captures(&expression);

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let division: i32 = left_value / right_value;
        expression = expression.replace(cap_expression, &division.to_string());

        /* For generate more details about its output we can use:
        println!("{:?} left: {}, right: {}.", caps, left_value, right_value); */
    };

    // Addition
    loop {
        // We use user data and generate a result
        let caps = re_addition.captures(&expression);

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition: i32 = left_value + right_value;
        expression = expression.replace(cap_expression, &addition.to_string());

        /* For generate more details about its output we can use:
        println!("{:?} left: {}, right: {}.", caps, left_value, right_value); */
    };

    // Subtraction
    loop {
        // We use user data and generate a result
        let caps = re_subtraction.captures(&expression);

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let subtraction: i32 = left_value - right_value;
        expression = expression.replace(cap_expression, &subtraction.to_string());

        /* For generate more details about its output we can use:
        println!("{:?} left: {}, right: {}.", caps, left_value, right_value); */
    };

    // We show our result
    println!("Your result is equal to: {}", expression);

}
