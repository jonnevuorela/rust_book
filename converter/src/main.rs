use std::io;

fn main() {
    let mut input = String::new();
    println!("Please provide a value for the converter as celcius.");
    io::stdin().read_line(&mut input).expect("Not valid");
    let input: f32 = input
        .trim()
        .parse()
        .expect("problem converting input to integer");

    println!("Value entered: {}", input);
    let converted_value: f32 = (input * 1.8) + 32.0;
    println!("Converted value is: {}", converted_value);
}
