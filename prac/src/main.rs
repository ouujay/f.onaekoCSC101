/*use std::io;
 
 fn main(){
    
    loop{
    println!("what minster are ypu in search of
        1 aigbogun alamba dauda
        2 murtaal afeez bandu
        3 okorocha calistus ogbona
        4 adewale jimoh akanbi
        5 ozazuwa faith etiye");
    let mut input1 = String::new();
    println!("type the number assigned to the ministers name to get futher information");
      io::stdin().read_line(&mut input1).expect("not a valid string");
      let input1:i32 = input1.trim().parse().expect("Not a valid number");

      
    if input1 == 1{
        println!("1. aigbogun alamba dauda\n-Ministry:Internal affairs-\nGeopolitical zone:South west ");

    }
    else if input1 == 2{
        println!("1.  Murtaal afeez bandu\n-Ministry:Justice-\nGeopolitical zone:North east ");


 }
  else if input1 == 3 {
        println!("1. Okorocha calistus ogbona \n-Ministry:Defence-\nGeopolitical zone:South south ");
    }
    else if input1 == 4 {
        println!("1. Adewale jimoh akanbi \n-Ministry:Power & steel-\nGeopolitical zone:South west ");
    }
    else if input1 == 5 {
        println!("1. ozazuwa adewale faith \n-Ministry:Petroleum-\nGeopolitical zone:South east ");
    }
  
   let mut input2 = String::new();
   println!("do you want to check for another commissioner--- tuye y or n");
      io::stdin().read_line(&mut input2).expect("not a valid string");
     
      if input2 == "y" {
        main();
      }
      else if input2 == "n" {
        println!("hope the information is useful & have a nice day");
      }
       break;
    }
}
*/
use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                                             
use std::fs::File;    
struct Particle {
  x: f32,
  y: f32,
  z: f32,
}

let data = vec![    Particle { x: 1.0, y: 2.0, z: 3.0}  ];

let mut f = File::create("output.vtk")? 

for thing in &data {
  write!(f, "{} {} {}", thing.x, thing.y, thing.z)?;
}