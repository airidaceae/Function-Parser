use std::str::Chars;
use text_io;

fn power(num1: f64, thing2: f64) -> f64{
    let num2 = thing2 as i32;
    let mut output = 1.0;
    for _ in 0..num2{
        output = output * num1; //this is a comment
    }
    return output;
}
fn evaluate(function: &str, x: f64 ) -> f64 {
    let mut substr: Vec<String> = Vec::new();
    let mut past = false;
    let mut token_list: Vec<Chars> = Vec::new();
    //println!("{}", function);

    let plus = function.find("+");
    let minus = function.find("-");
    let multiply = function.find("*");
    let divide = function.find("/");
    let exponent = function.find("^");
    let paren = function.find("(");


    if plus != None {
        substr.push(function.chars().skip(0).take(plus.unwrap()).collect());
        substr.push(function.chars().skip(plus.unwrap()).take(1).collect());
        substr.push(function.chars().skip(plus.unwrap() + 1).take(function.chars().count() - plus.unwrap()).collect());
        past = true;
        }
    else if minus !=None {
        substr.push(function.chars().skip(0).take(minus.unwrap()).collect());
        substr.push(function.chars().skip(minus.unwrap()).take(1).collect());
        substr.push(function.chars().skip(minus.unwrap() + 1).take(function.chars().count() - minus.unwrap()).collect());
        past = true;
    }
    else if multiply != None {
        substr.push(function.chars().skip(0).take(multiply.unwrap()).collect());
        substr.push(function.chars().skip(multiply.unwrap()).take(1).collect());
        substr.push(function.chars().skip(multiply.unwrap() + 1).take(function.chars().count() - multiply.unwrap()).collect());
        past = true;
    }
    else if divide != None {
        substr.push(function.chars().skip(0).take(divide.unwrap()).collect());
        substr.push(function.chars().skip(divide.unwrap()).take(1).collect());
        substr.push(function.chars().skip(divide.unwrap() + 1).take(function.chars().count() - divide.unwrap()).collect());
        past = true;
    }
    else if exponent != None{
        substr.push(function.chars().skip(0).take(exponent.unwrap()).collect());
        substr.push(function.chars().skip(exponent.unwrap()).take(1).collect());
        substr.push(function.chars().skip(exponent.unwrap() + 1).take(function.chars().count() - exponent.unwrap()).collect());
        past = true;
    }

    if past {
        match substr[1].as_str(){
            "+" => return evaluate(&substr[0], x) + evaluate(&substr[2], x),
            "-" => return evaluate(&substr[0], x) - evaluate(&substr[2], x),
            "/" => return evaluate(&substr[0], x) / evaluate(&substr[2], x),
            "*" => return evaluate(&substr[0], x) * evaluate(&substr[2], x),
            "^" => return power(evaluate(&substr[0], x), evaluate(&substr[2], x)),
            _ => return 0.0
        }

    }
    else if function == "x" {
        return x;
    }
    else {
        return function.parse().unwrap();
    }
}
fn main() {
    let mut function = String::from("");
    let mut input_x = String::from("");
    let mut precision = String::from("");

    println!("Hello, this program will solve for approximate tangent line at a point!");
    print!("Please input the function you are working with: ");
    function = text_io::read!();
    print!("Please input the x value you are searching for: ");
    input_x = text_io::read!();
    print!("How many decimal points of precision do you need? ");
    precision = text_io::read!();

    let input_x:f64 = input_x.parse().unwrap();
    let mut start = String::from("0.");
    let precision:i32 = precision.parse().unwrap();
    for _ in 0..precision + 1 {
        start.push_str("0");
    }
    start.push_str("1");
    let editor:f64 = start.parse().unwrap();
    let lower:f64 = input_x - editor;
    let upper:f64 = input_x + editor;
    println!("eval of f(x) = {}", evaluate(&function, input_x));
    //evaluate(function, upper);
    let estimated_derivative = (evaluate(&function, upper) - evaluate(&function, lower)) / (upper-lower);
    println!("eval of f'(x) = {}", estimated_derivative);
}
