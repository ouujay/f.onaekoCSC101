use std::io;

fn main() {

let mut input1  = String::new();
let mut input2  = String::new();

println!("---------------------\nEnter age");
 io::stdin().read_line(&mut input1).expect("Not a valid string");
let _age:i32 = input1.trim().parse().expect("Not a valid number");

println!("-----------------------\nAre you experienced (enter true or false)");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let _experience:bool = input2.trim().parse().expect("Not a valid ");

if _age >= 40 && _experience == true
{
    println!("----------------------\nYOUR ANNUAL INCENTIVE IS N1,560,000");
}
    else if _experience == true && _age >= 30 && _age < 40  
{
   println!("-------------------------\nYOUR ANNUAL INCENTIVE IS N1,480,000");
}
else if _experience == true && _age < 28 
{
    println!("-------------------------\nYOUR ANNUAL INCENTIVE IS N1,300,000");
}
else if _experience == false 
{
    println!("------------------------------\nYOUR ANNUAL INCENTIVE IS N100,000");

}

}
