 use std::io;
fn higherthan18(){
    let mut input1 = String::new();
    println!("if married ..type true");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _marital_status:bool = input1.trim().parse().expect("Not a valid ");
    if _marital_status == false{
         let mut input3 = String::new();
         let mut input2 = String::new();
        println!("What is your university");
        io::stdin().read_line(&mut input2).expect("not a valid string");
        println!("What is your course of study");
        io::stdin().read_line(&mut input3).expect("not a valid string");
        

    }
    else if _marital_status ==  true{
         let mut input4 = String::new();
        println!("do you have any offspring");
          io::stdin().read_line(&mut input4).expect("Not a valid string");
          let mut input5 = String::new();
          println!("where does your family live");
          io::stdin().read_line(&mut input5).expect("Not a valid string");

          }

    }
    fn less_than_18(){
        let mut input1 = String::new();
        println!("Has he/she written waec (enter true for yes ... other wise if no");
         io::stdin().read_line(&mut input1).expect("Not a valid string");
          let _waecstatus:bool = input1.trim().parse().expect("Not a valid ");
          if _waecstatus == true{
            println!("your school attended and class level ");
          }
          else if _waecstatus == false {
            println!("you are a fool");
          }



    }

fn main(){
loop {
let mut input1 = String::new();
println!("how many sibling do you have");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let _numofsibs:i32 = input1.trim().parse().expect("Not a valid number");

let mut input3 = String::new();
println!("enter their first name");
io::stdin().read_line(&mut input3).expect("not a valid string");
let mut input4 = String::new();
println!("enter age of sibling");
io::stdin().read_line(&mut input4).expect("not a valid string");
let _numofsibss:i32 = input1.trim().parse().expect("Not a valid number");



if _numofsibss >18  {
higherthan18();
}
else if _numofsibss <18 {
less_than_18();
}



let mut input2 = String::new();
   println!("have you finished entering all the values(y/n)");
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
