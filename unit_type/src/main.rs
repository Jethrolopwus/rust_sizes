// fn f2() {}

// unit types

fn f2() -> () {}

fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err(String::from("Error: division By Zero")),
        _ => {
            println!("The divsion is invalid");
            Ok(())
        }
    };
    answer
}


fn main() {
    let x = ();
    let y = f2();
    let z = println!("Hello Zero Sized");
    let mut vec: Vec<()> = Vec::with_capacity(0);
    vec.push(());
    vec.push(());
    vec.push(());

    assert_eq!(3, vec.len());
    println!("{}", vec.capacity());
}
