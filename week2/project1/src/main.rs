fn main()
{

    let x:f32 = 520000000.0;
    let y:f32 = 10.0;
    let z:f32 = 5.0;
    /*simple interest*/

   let P = x * (1.0+(y/100.0)) *z;
   println!("Amount is {}", P);
   //compound interest formula
   let compoundinterest = P - x;
    println!("Compound interest {}", compoundinterest);
}

