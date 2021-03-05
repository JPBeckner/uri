use std::io;

fn calcualte_volume(radius: f64) -> f64{
    // function to isolate the volume calculation
    let pi: f64 = 3.14159;  // pi value
    return (4.0/3.0) * pi * radius.powi(3);
}

fn main() -> io::Result<()>{
    let mut input = String::new();
    let stdin = io::stdin();
    // read the radius
    stdin.read_line(&mut input)?;
    // convert the string input to float
    let radius: f64 = input.trim().parse::<f64>().unwrap();

    // prints the calculated volume
    println!("VOLUME = {:.3}", calcualte_volume(radius));

    Ok(())
}
