fn main() {
    // initialize a mutable tuple 
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);
    
    println!("Original tuple   =  {:?}", mountain_heights);

    // change the 3rd and 4th lements of a mutable tuple 
    mountain_heights.2 = "lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}", mountain_heights);

}
