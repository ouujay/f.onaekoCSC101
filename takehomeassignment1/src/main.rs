    fn code_7 (){
  let no2 = vec!["audit services\n","Climate changeand sustainability\n","Financial accounts advisory\n","Forensic and integrity services\n","private client audit experience\n","Accounting link\n","Assurance\n"];
 let no1 = vec!["services done are as follows \n","-consulting\n","Analyticsconsultingservices\n","Customerexperience\n","Cybersecurity, strategy\n", "riskcompliance and resilience\n","Digitaltransformation\n","Risk consulting services\n","Supply chain and operations\n","Technologytransformation\n"];
 let consult = "Algbona Juliet";
 let consult1 ="b.sc";
 let consult2 ="consulting";
 let assurance ="akpevwe Iloka";
 let assurance1 ="hnd";
 let assurance2 ="Assurance"; 
       let mut file = File::create("name-aigbona _julie.txt") .expect("Error encountered while creating file!");
    writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",consult, consult1,consult2,no1);
    println!("------------------------------------------
              your preferred file  set has  been created
              ------------------------------------------ ");
            
         let mut file = File::create("akpevwe.Iloka.txt") .expect("Error encountered while creating file!");
      writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",assurance, assurance1,assurance2,no2);
    
        }
     
     
 fn code_8 (){
  let no2 = vec!["change management and experience\n","hr transformation\n","intgrated work force ability\n","learning and development consulting\n","recognition and reward Advisory\n", "work force analytics\n","people and work force\n"];
 let no1 = vec!["services done are as follows \n","-tax\n","tax functions\n","tax policy\n","global trade\n", "tax accounting\n","tax compliance","tax transaction\n"];
 let consult = "adamu sagamu ∩╮(︶︿︶)╭∩╮";
 let consult1 ="hnd ∩╮(︶︿︶)╭∩╮";
 let consult2 ="people and workforce ∩╮(︶︿︶)╭∩╮";
 let assurance ="gbenga daniels";
 let assurance1 ="hnd";
 let assurance2 ="tax"; 
           let mut file = File::create("adamu.sagamu.txt") .expect("Error encountered while creating file!");
    writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",consult, consult1,consult2,no2);
    println!("------------------------------------------
              your preferred file  set has  been created
              ------------------------------------------ ");
    
         let mut file = File::create("gbenga.daniels.txt") .expect("Error encountered while creating file!");
      writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",assurance, assurance1,assurance2,no1);
    
    
      
     }
    fn code_9 (){
  let no2 = vec!["corperate finances\n","diversement and carves out\n","sustainability and esg services\n","m&a advisory\n","m&a integration\n", "m&a technologies and tools\n","m&a advanced analytics\n"];
 let no1 = vec!["services done are as follows \n","strategy consulting\n","corporate and growth strategy\n","Transaction strategy and execution\n","restructuring and  turn around\n", "industry strategy\n","digital businessbuilding\n","commercial strategy\n"];
 let consult = "ehis ero";
 let consult1 ="m.sc";
 let consult2 ="strategy";
 let assurance ="maris akinsola ∩╮(︶︿︶)╭∩╮";
 let assurance1 ="m.sc ∩╮(︶︿︶)╭∩╮";
 let assurance2 ="Transactions and corporate finance"; 
           let mut file = File::create("ehis.ero.txt") .expect("Error encountered while creating file!");
    writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",consult, consult1,consult2,no1);
    println!("∩╮(︶︿︶)╭∩╮------------------------------------------
              your preferred file  set has  been created
              ------------------------------------------");
               let mut file = File::create("maria.akinsola.txt") .expect("Error encountered while creating file!");
      writeln!(&mut file, "{}\nqualification-{}\ndept-{}\n {:?}",assurance, assurance1,assurance2,no2);
  
        
  }

   use std::io;
   use std::io::Write;
   use std::fs::File;
use std::io::prelude::*;
    fn main(){
    loop{
    let mut input1 = String::new();
println!("who do you want to create a file for
           1= aigbona / akpervwe lloka
           2= ehis ero / gbenga daniels
           3= adamu sagamu / maria akinsola
              THE KEYS ARE THE numbers");
  io::stdin().read_line(&mut input1).expect("Not a valid string"); 
  let input1:i32 = input1.trim().parse().expect("Not a valid number"); 
  
  if input1 == 1 {
code_7();
  }
  else if input1 == 2 {
    code_8();
  }
else if input1 == 3{
    code_9();
}
else {
    println!("BABA GO SLEEP  ∩╮(︶︿︶)╭∩╮");
}

let mut input2 = String::new();
   println!("-----------------------------------------------------
             DO YOU WISH TO CREATE MORE FILE OF DIFFERENT STUDENTS
             ------------------------------------------------------
             TYPE 1 TO CREATE ANOTHER DESIRED STUDENT SET ∩╮(︶︿︶)╭∩╮
             ----------------------------------------------------
             TYPE 2 TO GO AND SLEEP");
      io::stdin().read_line(&mut input2).expect("not a valid string");
      let input2:i32 = input2.trim().parse().expect("Not a valid number"); 
      if input2 == 1 {
        main();
      }
      else if input2 == 2 {
        println!("thank you ╭∩╮");
        break;
      }
  }
}