fn main() {
    //create an empty vector"city"
    let mut city : Vec<String> = Vec::new();
    //print City Vector
    println!("The city vector has  elements {}",city.len());
    //push new elements into 
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Failed to read input");
    for count in 0..city_num  {
    let mut input2 = String::new();
    println!("Enter city {}", count+1 );
    std::io::stdin().read_line(&mut input2).expect("Failed to reas input");
    let new_city:String = input2.trim().parse().expect("Invalid input");
    city.push(new_city);
    }
    print!("Your preffred cities are:\n");
    let mut count=1;
    //loop to iterate elements in vector
    for i in city
    {
        // iterating through i on the vector 
        println!("{} {}", count, i);
        count+=1;
    }
}
