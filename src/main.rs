use bbow::Bbow;

fn main() {
    // println!("Hello")
    let mut my_bag = Bbow::new();
    my_bag = my_bag.extend_from_text("This is a te'st string");
    for key in my_bag.words(){
        println!("Key: {}", key);   
    }
    // let is_count = my_bag.count("is");
    // println!("'is' is in the list {} times", is_count)
    if my_bag.is_empty() {
        println!("bag is empty");
    }
    else{
        println!("bag is not empty");
        println!("there are {} words in the bag", my_bag.len());
        println!("total word count is: {}", my_bag.count());
    }
}