use std::io;
fn trapezium (){
     println!("Welcome you are using SB'S code");
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();
     println!("enter value for the height (in decimals for example 2.00)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");  
    let _height:f64 = input1.trim().parse().expect("Not a valid number");
     println!("enter value for the base1 (in decimals for example 2.00)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");  
    let _base1:f64 = input2.trim().parse().expect("Not a valid number");
    
    println!("enter value for the base2 (in decimals for example 2.00)");
    io::stdin().read_line(&mut input3).expect("Not a valid string");  
    let _base2:f64 = input3.trim().parse().expect("Not a valid number");

    let area:f64 = _height  * (_base1 + _base2) / 2.0;
    println!("the area of the trapezium is {}", area);
    }

fn rhombus() {
    println!("Welcome you are using SB'S code");
     let mut input1 = String::new();
     let mut input2 = String::new();
     
     println!("enter value for the diagonal1 (in decimals for example 2.00)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");  
    let _diagonal1:f64 = input1.trim().parse().expect("Not a valid number");
     println!("enter value for the diagonal2 (in decimals for example 2.00)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");  
    let _diagonal2:f64 = input2.trim().parse().expect("Not a valid number");


    let area:f64 = 0.5 * _diagonal1 * _diagonal2;
    println!("the area of the rhombus is : {}", area);
    }
fn parallelogram(){
     println!("Welcome you are using SB'S code");
     let mut input1 = String::new();
     let mut input2 = String::new();
     
     println!("enter value for the base (in decimals for example 2.00)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");  
    let _diagonal1:f64 = input1.trim().parse().expect("Not a valid number");
     println!("enter value for the altitude(in decimals for example 2.00)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");  
    let _diagonal2:f64 = input2.trim().parse().expect("Not a valid number");


    let area:f64 = 0.5 * _diagonal1 * _diagonal2;
    println!("the area of the rhombus is : {}", area);
}
fn cube(){
     println!("Welcome you are using SB'S code");
     let mut input1 = String::new();
     
     println!("enter value for the length(in decimals for example 2.00)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");  
    let _length:f64 = input1.trim().parse().expect("Not a valid number");
     
    let area:f64 = 5.0 * _length * _length;
    println!("the area of the cube is : {}", area);
}
fn volume_of_a_cylinder(){
     println!("Welcome you are using SB'S code");
     let mut input1 = String::new();
     let mut input2 = String::new();
     
     println!("enter value for the radius of cylinder (in decimals for example 2.00)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");  
    let _radius:f64 = input1.trim().parse().expect("Not a valid number");
     println!("enter value for the height(in decimals for example 2.00)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");  
    let _height:f64 = input2.trim().parse().expect("Not a valid number");


    let volume:f64 = 3.14 * _radius * _radius * _height;
    println!("the volume of the cylinder is : {}", volume);
}
fn main(){
    loop{
    let mut input1 = String::new();
println!("What complex calculation do you want to do(type the keys assigned)
           a= 1.Area of trapezium
           b= 2.Area of rhombus
           c= 3.Area of paralellogram
           d= 4.Area of cube formula
           e= 5.Volume of cylinder
           THE KEYS ARE THE LETTERS");
  io::stdin().read_line(&mut input1).expect("Not a valid string");  
  
  if input1 == "a" {
trapezium();
  }
  else if input1 == "b"{
    rhombus();
  }
else if input1 == "c"{
    parallelogram();
}
else if input1 == "d"{
    cube();

}
else if input1 == "e"{
    volume_of_a_cylinder();
}
let mut input2 = String::new();
   println!("have you finished all the calculations wanted to be carried out (y/n)");
      io::stdin().read_line(&mut input2).expect("not a valid string");
     
      if input2 == "n" {
        main();
      }
      else if input2 == "y" {
        println!("thank you");
        break;
      }

}
}
