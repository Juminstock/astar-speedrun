use regex::Regex;

// Math operations
fn math_operation(expression: Regex, mut operation: String, operator: &str) -> String {
    loop {
        let caps = expression.captures(operation.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let caps_expression = caps.get(0).unwrap().as_str();
        let caps_num1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let caps_num2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let result = match operator {
            "+" => caps_num1 + caps_num2,
            "-" => caps_num1 - caps_num2,
            "*" => caps_num1 * caps_num2,
            "/" => caps_num1 / caps_num2,
            _ => 0,
        };
        operation = operation.replace(caps_expression, &result.to_string());
    }
    operation
}

fn main() {
    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();

    // User input
    println!("Enter an expression to evaluate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input = math_operation(re_mul, input.clone(), "*");
    input = math_operation(re_div, input.clone(), "/");
    input = math_operation(re_add, input.clone(), "+");
    input = math_operation(re_sub, input.clone(), "-");

    // Show results

    println!("Result: {}", input);
}