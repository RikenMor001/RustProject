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
    
    println!("\nProgram completed successfully!");
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

