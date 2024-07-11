// Simple calculator using rust


use std::io;

fn main() {
    
    let mut rerun: bool = true;

    println!("\nℂ 𝔸 𝕃 ℂ 𝕌 𝕃 𝔸 𝕋 𝕆 ℝ");

    while rerun {
        // let mut choice_bool: bool = true;

        let  mut choice: i32;

        loop {
            // Ask user what arithmetic operation they would like to perform
            println!("\nPlease choose what basic arithmetic operation you would like to perform:");
            println!("\n1. Addition operation\n2. Subtraction operation\n3. Multiplication operation\n4. Division operation");
            println!("\nPlease select an option by inputing any number from 1 - 4: ");

            let mut choice_input = String::new();
            io::stdin()
                .read_line(&mut choice_input)
                .expect("Failed to read line");

            // Trim and parse the input
            match choice_input.trim().parse::<i32>() {
                Ok(num) => {
                    choice = num;
                    if choice < 1 || choice > 4 {
                        println!("\n'{}' is not a valid option!", choice);
                        println!("\nPlease input a number between 1 - 4!\n");
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    println!("\nNot a valid option!");
                    println!("\nPlease input a NUMBER between 1 - 4!\n");
                    continue;
                }
            }


        }
        

        // Print the option chosen
        println!("\nYou chose option {}.", choice);

        let num1: f64;
        let num2: f64;
        loop {
            // First number
            println!("\nInput the first number: ");

            let mut num1_input = String::new();
            io::stdin()
                .read_line(&mut num1_input)
                .expect("Failed to read line.");


            // Trim and parse the input
            match num1_input.trim().parse::<f64>() {
                Ok(num) => {
                    num1 = num;
                    break;
                    }
                
                Err(_) => {
                    println!("\nPlease input a NUMBER!");
                    continue;
                }
            }
        }

        loop {    
            // Second number
            println!("\nInput the Second number: ");

            let mut num2_input = String::new();
            io::stdin()
                .read_line(&mut num2_input)
                .expect("Failed to read line.");

            // Trim and parse the input
            match num2_input.trim().parse::<f64>() {
                Ok(num) => {
                    num2 = num;
                    break;
                    }
                
                Err(_) => {
                    println!("\nPlease input a NUMBER!");
                    continue;
                }
            }
        }

        println!("");
        if choice == 1 {
            let add = num1 + num2;
            println!("Result: ({}) + ({}) = {}", num1, num2, add);
        } else if choice == 2 {
            let sub = num1 - num2;
            println!("Result: ({}) - ({}) = {}", num1, num2, sub);
        } else if choice == 3 {
            let mul = num1 * num2;
            println!("Result: ({}) x ({}) = {}", num1, num2, mul);
        } else if choice == 4 {
            let div = num1 / num2;
            println!("Result: ({}) ÷ ({}) = {}", num1, num2, div);
        } 

        println!("\nWould you like to rerun? (y/n): ");

        let mut rerun_input = String::new();
        io::stdin()
            .read_line(&mut rerun_input)
            .expect("Failed to read line.");

        rerun_input = rerun_input.trim().to_lowercase();
        if rerun_input == "n" {
            rerun = false;
        }
    }

    println!("\nEnd of program.\n")

    
}
