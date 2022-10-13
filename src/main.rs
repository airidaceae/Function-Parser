/* Iris Pupo
 * sudo-iris
 * To-do:
 * Parenthesis support
 * clean up output formatting
 * improve multi-digit parsing support
 * coefficients????
 *
 */
use std::io::{stdout, Write};

trait FindClose {
    fn find_close(self, pos: usize) -> Option<usize>;
}
impl FindClose for &str{
    fn find_close(self, pos: usize) -> Option<usize> {
        let mut iterator: usize = 0;
        let mut paren_count: i32 = 0;
        for part in self.chars() {
            if iterator >= pos {
                if part == '(' {
                    paren_count += 1;
                } 
                else if part == ')' {
                    paren_count -= 1;
                    if paren_count == 0 {
                        return Some(iterator);
                    }
                }
            }
            iterator += 1;
        }
        return None;
    }
}

fn token_finder(expression: &str, token_set: Vec<char>) -> Option<usize> {
    let mut position: usize = 0;
    for character in expression.chars() {
        for token in &token_set {
             if token == &character {
                return Some(position);
            }
        }
        position += 1;
    } 
    return None;
}

fn parse(expression: &str) -> Vec<String>{
    let mut token_layer: [Vec<char>; 4] = Default::default();
    token_layer[0].push('(');
    token_layer[1].push('+');
    token_layer[1].push('-');
    token_layer[2].push('*');
    token_layer[3].push('^');
    let mut split_pos: Option<usize>;

    let mut substr: Vec<String> = Vec::new();
    for layer in &token_layer{
         split_pos = token_finder(expression, layer.to_owned());
        if split_pos.is_some(){
            let split_pos: usize = split_pos.unwrap();
            if layer.to_owned() == token_layer[0]{
           
            }
            else{
                substr.push(expression.chars().take(split_pos).collect()); 
                substr.push(expression.chars().skip(split_pos).take(1).collect()); 
                substr.push(expression.chars().skip(split_pos + 1).take(expression.chars().count() - split_pos).collect()); 
            }
        }
    }
    return substr;
}


//this function checks if the expression is able to be parsed
fn can_be_parsed(expression: &str) -> bool{ 
    let mut token_list: Vec<Option<usize>> = Vec::new();
    token_list.push(expression.find('+'));
    token_list.push(expression.find('-'));
    token_list.push(expression.find('*'));
    token_list.push(expression.find('/'));
    token_list.push(expression.find('^'));
    token_list.push(expression.find('('));
    token_list.push(expression.find(')'));
    for token in token_list{
        if token.is_some(){
            return true;
        }
    }
    return false;
}

fn evaluate(expression: &str, x: f64 ) -> f64 {
    let mut substr: Vec<String> = Vec::new();
    let past: bool;
    println!("{}", expression); 
    if can_be_parsed(expression) {
        substr = parse(expression);
        past = true;
    }
    else{
        past = false;
    }
    if past {
        match substr[1].as_str(){
            "+" => return evaluate(&substr[0], x) + evaluate(&substr[2], x),
            "-" => return evaluate(&substr[0], x) - evaluate(&substr[2], x),
            "/" => return evaluate(&substr[0], x) / evaluate(&substr[2], x),
            "*" => return evaluate(&substr[0], x) * evaluate(&substr[2], x),
            "^" => return evaluate(&substr[0], x).powf(evaluate(&substr[2],x)),
            _ => return 0.0
        }

    }
    else if expression == "x" {
        return x;
    }
    else {
        return expression.parse().unwrap();
    }
}

fn main() {
    let mut function = String::from("");
    let mut input_x = String::from("");
    let mut precision = String::from("");

    println!("Hello, this program will solve for approximate tangent line at a point!");
    print!("Please input the function you are working with: ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut function).unwrap();
    print!("Please input the x value you are searching for: ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut input_x).unwrap();
    print!("How many decimal points of precision do you need? ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut precision).unwrap();

    let precision:i32 = precision.trim().parse().unwrap();
    let input_x:f64 = input_x.trim().parse().unwrap();
    function = function.trim().to_string();

    let mut start = String::from("0.");
    for _ in 0..precision + 1 {
        start.push_str("0");
    }
    start.push_str("1");
    let editor:f64 = start.parse().unwrap();
    let lower:f64 = input_x - editor;
    let upper:f64 = input_x + editor;

    println!("eval of f({}) = {}", input_x, evaluate(&function, input_x));
    let estimated_derivative = (evaluate(&function, upper) - evaluate(&function, lower)) / (upper-lower);
    println!("eval of f'({}) = {}", input_x, estimated_derivative);
}
