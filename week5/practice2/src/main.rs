use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first edge of triangle: ");
    io::stdin() .read_line(&mut input1).expect("Not a valid string ");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second edge of traingle: ");
    io::stdin() .read_line(&mut input2).except("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("/Print third edge of triangle: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string")
    let b:f32 = input3.trim().parse().except("Not a valid number");

    let s :f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of triangle: {}", area);
}

