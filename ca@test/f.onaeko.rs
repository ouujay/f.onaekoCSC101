use std::io
 
 fn GeoPo_meger(){
 	
 	loop{
 	println!("what minster are ypu in search of
 		1 aigbogun alamba dauda
 		2 murtaal afeez bandu
 		3 okorocha calistus ogbona
 		4 adewale jimoh akanbi
 		5 ozazuwa faith etiye")
 	let mut input1 = String::new();
 	println!("type the number assigned to the ministers name to get futher information")
 	  io::stdin().read_line(&mut input1).expect("not a valid string");
 	  let input:i32 = input1.trim().parse().expect("Not a valid number");

 	  
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
   println!("do you want to check for another commissioner--- tuye y or n")
 	  io::stdin().read_line(&mut input2).expect("not a valid string");
 	 
 	  if input2 == y {
 	  	GeoPo_meger();
 	  }
 	  else if input2 == n {
 	  	println!("hope the information is useful & have a nice day");
 	  	break;
 	  }
 	}







 fn Pub_service(){
 	println!("1:office administrator
 		      2:accademic
 		      3:lawyer
 		      4:teacher
 		      ")
 	let mut input  = String::new();
 	println!("enter your public service office (the number assigned to the public servise office)")
 	 io::stdin().read_line(&mut input).expect("Not a valid string");
 	  let number:i32 = input1.trim().parse().expect("Not a valid number")

 	  if input == 1{
 	  	println!("welcome administrator");
 	  	let mut input1 = String::new;
 		println!("enter amount of years ");
 		 	 io::stdin().read_line(&mut input).expect("Not a valid string");
 	  let number:i32 = input1.trim().parse().expect("Not a valid number")

 		if input1 >= 1 || input <= 2{
 		println!("you are an intern");
 	}
     
 	
 	else if input1 >= 3|| input1 <= 5{
 		println!("administrator");
 

 	  }
 	  else if input1 =>6 || input1 <= 8{
 	  	println!("senior administrator");

 	  }
	
 	else if input1 =>8 || input1 <=10{
 		println!("office manager");
 

 	  }
 	  else if input1 => 10 || input1 <= 13{
 	  	println("direcrtor");
 	  }
 	  	
 	else if input1 => 14{
 		println!("CEO");
 

 	  }
 	  	
     else if input == 2{
 	  	println!(academic);
 	  	let mut input1 = String::new;
 		println!("enter amount of years ");
 		 	 io::stdin().read_line(&mut input).expect("Not a valid string");
 	  let number:i32 = input1.trim().parse().expect("Not a valid number")

 		if input1 >= 1 || input <= 2{
 		println!("you are nothing");
 	}
     
 	
 	else if input1 >= 3|| input1 <= 5{
 		println!("reasearch analist");
 

 	  }
 	  else if input1 =>6 || input1 <= 8{
 	  	println!("senior administrator");

 	  }
	
 	else if input1 =>8 || input1 <=10{
 		println!("office manager");
 

 	  }
 	  else if input1 => 10 || input1 <= 13{
 	  	println("direcrtor");
 	  }
 	  	
 	else if input1 => 14{
 		println!("CEO");
 


 if input == 3{
 	  	println!("welcome administrator");
 	  	let mut input1 = String::new;
 		println!("enter amount of years ");
 		 	 io::stdin().read_line(&mut input).expect("Not a valid string");
 	  let number:i32 = input1.trim().parse().expect("Not a valid number")

 		if input1 >= 1 || input <= 2{
 		println!("you are an intern");
 	}
     
 	
 	else if input1 >= 3|| input1 <= 5{
 		println!("administrator");
 

 	  }
 	  else if input1 =>6 || input1 <= 8{
 	  	println!("senior administrator");

 	  }
	
 	else if input1 =>8 || input1 <=10{
 		println!("office manager");
 

 	  }
 	  else if input1 => 10 || input1 <= 13{
 	  	println("direcrtor");
 	  }
 	  	
 	else if input1 => 14{
 		println!("CEO");
 }
  if input == 1{
 	  	println!("welcome administrator");
 	  	let mut input1 = String::new;
 		println!("enter amount of years ");
 		 	 io::stdin().read_line(&mut input).expect("Not a valid string");
 	  let number:i32 = input1.trim().parse().expect("Not a valid number")

 		if input1 >= 1 || input <= 2{
 		println!("you are an intern");
 	}
     
 	
 	else if input1 >= 3|| input1 <= 5{
 		println!("class teacher");
 

 	  }
 	  else if input1 =>6 || input1 <= 8{
 	  	println!("snr teachet");

 	  }
	
 	else if input1 =>8 || input1 <=10{
 		println!("lead teacher");
 

 	  }
 	  else if input1 => 10 || input1 <= 13{
 	  	println("deputy");
 	  }
 	  	
 	else if input1 => 14{
 		println!("principal");
 
}
fn main(){
	println!("what division do you want to access");
	let mut input1 = String::new;
	println!("type y for GeoPo_meger or type n for  Pub_service");
	 io::stdin().read_line(&mut input1).expect("not a valid string");
	 if input1 == y {
	 	GeoPo_meger();
	 }
	 else if input1 == n {
	 	 Pub_service();
	 }
}


 	