use std::{io, panic};

fn main() {
    println!("Enter a temperature to be converted. Write it as 0C or 32F");

    let msg = "Failed to parse value";

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect(msg);
    guess = guess.trim().to_string();

    let units = guess.pop().expect(msg);
    let value: i32 = guess.trim().parse().expect(msg);

    // println!("Value: {value}");
    // println!("Units: {units}");

    let converted = match units {
        'C' => value * 9 / 5 + 32,
        'F' => (value - 32) * 5 / 9,
        _ => panic!("Incorrect Units"),
    };

    let other_units = match units {
        'C' => 'F',
        'F' => 'C',
        _ => panic!("Critical failure."),
    };

    println!("The converted value is {converted}{other_units}");
}
