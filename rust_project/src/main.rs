use std::io;

fn main() {
    println!("Welcome to Simple Calculator!");
    println!("This is a basic Rust program to learn the fundamentals.");
    
    let mut counter = 0;
    let name = "Learner";
    
    println!("Hello, {}! Let's count from 0 to 5:", name);
    
    while counter <= 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
    
    let result = add_numbers(10, 20);
    println!("10 + 20 = {}", result);
    
    println!("\nEnter a number to multiply by 2:");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    match input.trim().parse::<i32>() {
        Ok(number) => {
            let doubled = multiply_by_two(number);
            println!("{} * 2 = {}", number, doubled);
        }
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
        }
    }
    
    let numbers = [1, 2, 3, 4, 5];
    println!("\nArray elements:");
    for (index, &value) in numbers.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
    
    println!("\nLet's try subtraction!");
    
    let mut inputForSubtractionA = String::new();
    let mut inputForSubtractionB = String::new();

    println!("Enter a number for the first operand: ");
    io::stdin()
        .read_line(&mut inputForSubtractionA)
        .expect("Failed to read input number for the first operand");

    println!("Enter a number for the second operand: ");
    io::stdin()
        .read_line(&mut inputForSubtractionB)
        .expect("Failed to read input number for the second operand");

    let first_number = match inputForSubtractionA.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input for first number! Please enter a valid number.");
            return;
        }
    };

    let second_number = match inputForSubtractionB.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input for second number! Please enter a valid number.");
            return;
        }
    };

    let subtraction_result = subtract_numbers(first_number, second_number);
    println!("{} - {} = {}", first_number, second_number, subtraction_result);

    println!("\nProgram completed successfully!");

    println!("\n Let's try division!");

    let mut inputForDivisionForFirstOperand = String::new();
    let mut inputForDivisionForSecondOperand = String::new();

    println!("Enter a number for the first operant: ");
    io::stdin()
    .read_line(&mut inputForDivisionForFirstOperand)
    .expect("Failed to read input number for the first operand");    

    println!("Enter a number for the second operand:");

    io::stdin()
    .read_line(&mut inputForDivisionForSecondOperand)
    .expect("Failed to read input number for the second operand");

    let first_number = match inputForDivisionForFirstOperand.trim().parse::<i32>(){
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input for first number! Please enter a valid number.");
            return;
        }
    };
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

fn subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}

fn divide_numbers(a: i32, b: i32) -> i32{
    a / b
}
