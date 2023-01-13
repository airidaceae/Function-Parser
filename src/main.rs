// Iris Pupo
// airidaceae
 
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
                        //println!("returning some");
                        return Some(iterator);
                    }
                }
            }
            iterator += 1;
        }
        //println!("returning none\n iterator is {}", iterator);
        None
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
    None
}

fn parse(expression: &str) -> Vec<&str>{

    //layers are neccesary to make sure one token from a pemdas layer does not have higher priority 
    let mut _token_layer = [
        vec!['+', '-'], 
        vec!['*', '/'],
        vec!['^'],
    ];
    
    let mut split_pos: Option<usize>;
    let mut substr: Vec<&str> = Vec::new();
    for layer in _token_layer {
         split_pos = token_finder(expression, layer.to_owned());
        if split_pos.is_some(){
            let split_pos = split_pos.unwrap();
           
            //return the parts of the substring that will be evaluated.
            substr.push(&expression[0..split_pos]);
            //println!("push 1 is {}", substr[0]);
            substr.push(&expression[split_pos..(split_pos+1)]);
            //println!("push 2 is {}", substr[1]);
            substr.push(&expression[(split_pos + 1)..(expression.chars().count())]);
            //println!("push 3 is {}", substr[2]);
        }
    }
    substr
}

//this function checks if the expression is able to be parsed, probably useless
fn can_be_parsed(expression: &str) -> bool{
    //println!("current expression is {}",expression);
    let mut _token_list: Vec<char> = vec!['+', '-', '*', '/', '^', '(', ')'];
    for token in _token_list{
        if expression.find(token).is_some(){
            return true;
        }
    }
    false
}

fn evaluate(expression: &str, x: f64 ) -> f64 {
    let mut substr: Vec<&str> = Vec::new();
    let past: bool;

    //println!("{}", expression); 
    let paren = expression.find("(");
    let mut evaluated_expression = String::from("");
    if paren.is_some(){
        let paren = paren.unwrap();
        let close_pos = expression.find_close(paren);
        if close_pos.is_some(){ //evaluates parenthesis by seperating it into the parts around the
                                //parenthesis and concatinating it with the result of evaluating
                                //the inner value
            let close_pos = close_pos.unwrap();

            evaluated_expression.push_str(&expression[0..paren]);
            //println!("ppush 1 is {}", evaluated_expression);
            evaluated_expression.push_str(&evaluate(&expression[(paren + 1)..(close_pos - paren )], x).to_string());
            //println!("ppush 2 is {}", evaluated_expression);
            evaluated_expression.push_str(&expression[(close_pos + 1)..(expression.chars().count())]);
            //println!("ppush 3 is {}", evaluated_expression);
            return evaluate(&evaluated_expression, x);
        }
        //println!("LINE 114 CODE ERROR");
        past = false;
    }
    else if can_be_parsed(expression) {
        //println!("expression being parsed is {}",expression);
        substr = parse(expression);
        //println!("substring 0 = {}", substr[0]);
        //println!("substring 1 = {}", substr[1]);
        //println!("substring 2 = {}", substr[2]);

        //println!("past");
        past = true;
    }
    else{
        //println!("did not pass");
        past = false;
    }

    if past {
        let lhs: f64 = evaluate(&substr[0], x);
        let rhs: f64 = evaluate(&substr[2], x);
        match substr[1]{
            "+" => lhs + rhs,
            "-" => lhs - rhs, 
            "/" => lhs / rhs,
            "*" => lhs * rhs, 
            "^" => lhs.powf(rhs), 
            _   => 0.0
        }

    }
    else if expression == "x" {
        //println!("returning x is {}", x);
        x
    }
    else {
        //println!("expression is {} before parse and return", expression);
        expression.parse().unwrap()
    }
}

//a functiont to format the output bc i could not find a better way to do it
fn format(input: f64, precision: i32) -> f64{
    let mut num = input;
    let powered: f64 = 10.0;
    let powered = powered.powi(precision);
    num *= powered;
    num = num.round();
    num /= powered;
    num
}


// implement this you gay whore ^-^
/*fn derive(expression: &str, x: f64, degree: i32) -> f64{
    if degree == 0{
        return evaluate(expression, x)
    
    } else {
        return


    }

    todo!();
}*/

fn main() {
    let mut function = String::from("");
    let mut input_x = String::from("");
    let mut precision = String::from("");

    println!("This program solves for apporoximate tangent line at a point :3");
    print!("Input the function you are working with: ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut function).unwrap();
    print!("Input the x value you are searching for: ");
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

    println!("f({}) = {}", input_x, format(evaluate(&function, input_x), precision));
    let estimated_derivative = (evaluate(&function, upper) - evaluate(&function, lower)) / (upper-lower);
    println!("f'({}) = {}", input_x, format(estimated_derivative, precision));
}
