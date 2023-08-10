use std::io;

fn main() {
    
        println!("Welcome. Please choose first number.");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .expect("Failed to read line");




    let number1: f64 = match input.trim().parse() {
        Ok(num) => num,
         Err(_) => {
            println!("Invalid input. Please enter a valid floating-point number.");
                return;
            }
        };




        println!("Choose the second number:");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    
    
    
    
        let number2: f64 = match input.trim().parse() {
            Ok(num) => num,
             Err(_) => {
                println!("Invalid input. Please enter a valid floating-point number.");
                 return;
                }
            };

    
    
    //println!("Sonuç: {}", calculate(Operation::Add(number1,number2)));
    println!("Please choose operation");
    let mut input = String::new();
    
    let ope = input.trim();

    let op_enum = match ope {
        "+" => Operation::Add(number1,number2),
        "-" => Operation::Substract(number1,number2),
        "*" => Operation::Multiply(number1,number2),
        "/" => Operation::Divide(number1,number2),
        _ => {
            println!("Invalid operation. Please enter one of the following: +, -, *, /");
            return;
        }
    };

    println!("Your result: {}", calculate(op_enum));
}

enum Operation{
    Add(f64,f64),
    Substract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(op: Operation) -> f64{
    match op{
        Operation::Add(num1,num2) => num1 + num2,
        Operation::Substract(num1,num2) => num1 - num2,
        Operation::Multiply(num1,num2) => num1 * num2,
        Operation::Divide(num1,num2) => num1/num2,
    }

}
