// use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome Please enter and expression ");

    let expression = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") {
            break;
        }
        enum Token {
            Number(f64),
            Operator(char),
        }
        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

        for item in parts.iter() {
            //Attempt to parse the string
        }

        // Use if let to handle the Option results safely.
        // if parts.len() != 3 {
        //     eprintln!(
        //         "Error: Invalid input format. Please use 'number operator number' (e.g., 10 * 5)."
        //     );
        //     continue; // Exit if format is wrong
        // }

        // // let expression: &str = parts.get(1).expect("Expected an expression at index 1");
        // let first_num_opt: Option<f64> = parts.get(0).and_then(|&s| s.parse::<f64>().ok());

        //
        // // Use .map(|&s| s) or .map(|v| *v) to convert Option<&&str> to Option<&str>
        // // The compiler's suggestion .map(|v| &**v) is also correct but `map(|&s| s)` is often more readable here.
        // let operator_opt: Option<&str> = parts.get(1).map(|&s| s); // Corrected line
        //                                                            //
        // let second_num_opt: Option<f64> = parts.get(2).and_then(|&s| s.parse::<f64>().ok());
        //
        // let output: f64 = match (first_num_opt, operator_opt, second_num_opt) {
        //     (Some(num1), Some(op), Some(num2)) => {
        //         match op {
        //             "*" => num1 * num2,
        //             "+" => num1 + num2,
        //             "-" => num1 - num2,
        //             "/" => {
        //                 if num2 != 0.0 {
        //                     num1 / num2
        //                 } else {
        //                     println!("Error: Division by zero is not allowed.");
        //                     continue; // Exit if division by zero
        //                 }
        //             }
        //             _ => {
        //                 println!(
        //                     "Error: Invalid operator '{}'. Supported operators are: +, -, *, /",
        //                     op
        //                 );
        //                 std::f64::NAN
        //             }
        //         }
        //     }
        //     _ => {
        //         // This arm catches any case where first_num_opt, operator_opt, or second_num_opt is None
        //         eprintln!("Error: Could not parse numbers or operator. Make sure they are valid.");
        //         std::f64::NAN // Indicate a general parsing/input error
        //     }
        // };
        // if output.is_nan() {
        //     println!("Calculation failed.");
        // } else {
        //     println!("Result: {}", output);
        // }
        // hello
    };
}
