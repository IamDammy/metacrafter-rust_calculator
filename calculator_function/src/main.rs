use std::io;

fn main() {
    println!("A metacrafter calculator built with Rust Programming Language");
    loop{
// we write a loop function to get the information to the user
println!("Please select an operation");
println!("1. Addition (+)");
println!("2. Subtraction (-)");
println!("3. Multiplication (*)");
println!("4. Division (/)");
println!("5. To QUIT or Exit the program");

// we make the users choice to be mutable
let mut user_input = String::new();
//the we use the I/O to read lines from the user_input
io::stdin().read_line(&mut user_input).expect("Failed to read line");

//since our user input is mutable, then we parse() the arguments to covert to type i32(signed type)
let user_input: i32 = match user_input.trim().parse() {
    Ok(num) => num,
    Err(_) => {//the is if error, 
        println!("Invalid input! Please enter a number.");
        continue;
    }
};

if user_input == 5 {
    println!("Goodbye!");
    break;
}
 
println!("Enter the first number");
let mut number_1 = String::new();
io::stdin().read_line(&mut number_1).expect("Failed to read line");
let number_1:f64 = match number_1.trim().parse(){
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input! Please enter a number");
        continue;
    }
};

println!("Enter the second number");
let mut number_2:String = String::new();
io::stdin().read_line(&mut number_2).expect("Failed to read line");
let number_2:f64 = match number_2.trim().parse(){
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input! Please enter a number");
        continue;
    }
};

match user_input {
    1 => println!("Result: {}", number_1 + number_2),
    2 => println!("Result: {}", number_1 - number_2),
    3 => println!("Result: {}", number_1 * number_2),
    4 => {
        if number_2 == 0.0 {
            println!("Error: division by 0 is not allowed in mathematics");
        } else {
            println!("Result: {}", number_1 / number_2);
        }
    }
  _ => println!("Invalid user input! please choose a number from 1 t0 5")
}

    }
}
