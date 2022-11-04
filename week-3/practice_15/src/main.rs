fn main() {
    //  basic formating 
    println!("{} is from {}", "brad","mars");
    //Postional arguments 
    println!("{0} is from {1} and {0} likes to {2} and {3}","Brad","Mars","code", "suck balllllsssssssssssss and penisssssssssssss");
    // named arguments 
    println!("{name} likes to play {activity}",name = "john", activity = "football");
    // place holders
    println!("Binary:{:b}  Ocatal:{:o}  hex {:h}", 10, 10, 10);
}
