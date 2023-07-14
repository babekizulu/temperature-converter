use std::io::stdin;
fn celsius_to_fahrenheit(c: f32) -> f32 {
    let a: f32 = 1.8 * c;
    let b: f32 = 32.0;
    let f: f32 = a + b;
    f
}
fn fahrenheit_to_celsius(f: f32) -> f32 {

    let a: f32 = f - 32.0;
    let b: f32 = 5.0/9.0;
    let c: f32 = a * b;
    c
}
fn main() {
    let mut prompt = String::new();
    let mut temp = String::new();
    let err = "Could not read line";
    let err_2 = "Please enter a valid temperature (must be a number)";
    let err_3 = "Error: invalid selection (select C or F)";
    println!("TEMPERATURE CONVERTER");
    println!("Are you converting to celsius or fahrenheit?(C/F):");
    stdin().read_line(&mut prompt).expect(err);
    if prompt.trim().to_string().to_lowercase() == "c" {
        println!("Enter temperature (F):");
        stdin().read_line(&mut temp).expect(err);
        let temp: f32 = temp.trim().parse().expect(err_2);
        println!("{}", fahrenheit_to_celsius(temp));
    }
    else if prompt.trim().to_string().to_lowercase() == "f" {
        println!("Enter temperature (C):");
        stdin().read_line(&mut temp).expect(err);
        let temp: f32 = temp.trim().parse().expect(err_2);
        println!("{}", celsius_to_fahrenheit(temp));
    }
    else {
        println!("{err_3}");
    }

}
