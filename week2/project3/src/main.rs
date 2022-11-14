fn main() {
    //inputting values
    let a:i32 = 210000;
    let b:i32 = 5;
    let c:i32 = 3;
    //price of the item 
    let d = 1 - (b/100);
   
    let compoundinterest = a * d;
    println!("Compound interest is {}", compoundinterest); 
}
