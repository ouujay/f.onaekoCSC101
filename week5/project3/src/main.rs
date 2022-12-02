use std::io;

fn main(){
    
    println!("-----------------------------------------------------------------\nTHESE ARE THE AVAILABLE ORDERS
        MENU                                     -PRICE
        POUNDO YAM/EDINKAINKO SOUP               -3,200
        AMALA & EWEDU SOUP                       -3,000
        EBA & EGUSI SOUP                         -2,000
        WHITE RICE & STEW                        -2,500");

    


      println!("-------------------------------------------------------------------------------------------------------------------
       \nWHAT DO YOU WANT TO ORDER ......SOFT MANS THERE IS GOING TO BE DISCOUNT IF YOUR ORDER IS MORE THAN 10,000. DON'T MISS OUT");
       println!(" KEYS
                  p = pounded yam / edikainko soup
                  a = amala & ewedu soup
                  e = EBA & EGUSI SOUP 
                  w = white rice & stew");
       let p =  " POUNDO YAM/EDINKAINKO SOUP";  
       let a = "AMALA & EWEDU SOUP";                   
       let e = "EBA & EGUSI SOUP";                      
       let w =  "WHITE RICE & STEW";        
       


let mut enter_your_order = String::new();
println!("Enter your order:");
io::stdin().read_line(&mut enter_your_order).expect("Failed to read input");
if enter_your_order == "p"{
    let mut quantity_p = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut quantity_p).expect("Failed to read input");
    let quantity_p:i32 = quantity_p.trim().parse().expect("Invalid input");
    let a = quantity_p*3200;  
    println!("your price is {}", a);

 
}
else if enter_your_order == "a"{
    let mut quantity_a = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut quantity_a).expect("Failed to read input");
    let quantity_a:i32 = quantity_a.trim().parse().expect("Invalid input");
        let c = quantity_a*3000;
    println!("your price is {}", c);


 
}
else if enter_your_order == "e"{
    let mut quantity_e = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut quantity_e).expect("Failed to read input");
    let quantity_e:i32 = quantity_e.trim().parse().expect("Invalid input");
        let b = quantity_e*2000;  
    println!("your price is {}", b);

 
}
else if enter_your_order == "w"{
    let mut quantity_w = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut quantity_w).expect("Failed to read input");
    let quantity_w:i32 = quantity_w.trim().parse().expect("Invalid input");
        let d = quantity_w*2500;  
    println!("your price is {}", w);
 
}
}                   
