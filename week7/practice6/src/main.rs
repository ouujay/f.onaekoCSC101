fn main() {
   // initialization of tuple with data type 
   let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
   println!("Tuple contents = {:?}", datatype_tuple);

   // intialization of tuple without data type
let no_datatype_tuple = ("Rust", "fun", 100);
   println!("Tuple contents = {:?}",no_datatype_tuple);

   //accessing tuple element at index 0
   println!("Value at Index 0 ={}", datatype_tuple.0);

//accesing tuple element at index 1
println!("Value at index 1 = {}", datatype_tuple.1);

//accesing tuple element at index 2
println!("Value at index 2 = {}", no_datatype_tuple.2);
}



