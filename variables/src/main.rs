fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const SEC_IN_HOUR: u32 = 60 * 60;
    println!("There are {SEC_IN_HOUR} seconds in an hour.");

    let y: isize = 10;
    println!("The value of y is: {y}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Characters: {c} {z} {heart_eyed_cat}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    
}