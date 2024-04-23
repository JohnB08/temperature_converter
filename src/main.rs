use std::io;

fn main() {
    /* This is a loop that runs until prompted to end by user.
    We use the std::io library to read commands from the terminal to do actions in the loop, or terminate the loop.
     */
    loop {
        /* The first message prompts the user to input either F or C */
        println!("Enter F to convert from celcius to fahrenheit, or C to convert from fahrenheit to celcius:");
        /* We then declare a new mutable string called conversion_type */
        let mut conversion_type = String::new();
        /* We use the io stdin() to read the user inserted line from the console, and try to insert the buffer into conversion_type.
        if it fails we throw an exception. */
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line.");
        /* We account for whitespace and capital letters. */
        conversion_type = conversion_type.trim().to_uppercase();
        /* Here we check for if conversion_type is of the correct letter type with an early continue back to the start of the loop. */
        if conversion_type != "F" && conversion_type != "C"{
            /* If conversion_type is of the wrong input, we restart the loop. */
            println!("Invalid conversion type entered.");
            continue;
        } else {
            /* We prompt the user to insert what tempereature they want converted. */
            println!("Enter the temperature to be converted to {}:", conversion_type);
            /* We initialize a new string to act as buffer. */
            let mut temperature = String::new();
            /* We use the io::stdin to capture the users input, and try to move it into temperature. If it fails we throw an exception.  */
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read temperature.");
            /* Here we try to parse the temperature into a floating point. if it fails, the user hasn't given us a number, and we throw and exception. */
            let temperature: f64 = temperature.trim().parse().expect("Please enter a valid number.");
            /* Here we do the conversion with an if/else statemenet, given we know conversion_type can only be either "F" or "C". */
            let converted_temperature = if conversion_type == "F" {
                (temperature*9.0/5.0)+32.0
            } else {
                (temperature-32.0)*5.0/9.0
            };
            /* Once that is done, we serve the converted number to the user. */
            println!("The converted temperature is {}°{}", converted_temperature, conversion_type);
        }
        /* Once conversion is completed, we prompt the user if they want to continue with a y/n answer. */
        println!("Do you want to do another conversion? (y/n)");
        let mut answer = String::new();
        /* Here we read the user input, and tries to move the buffer into answer. if it fails we throw an exception.*/
        io::stdin().read_line(&mut answer).expect("Failed to read the line.");
        /* If the answer is n or no (case insensitive due to eq_ignore_ascii_case) we break the loop. */
        if answer.trim().eq_ignore_ascii_case("n") || answer.trim().eq_ignore_ascii_case("no"){
            break;
        }
    }
}
