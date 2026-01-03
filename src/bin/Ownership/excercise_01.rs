//Create a function that takes ownership of a string and prints it.
use String;

fn main (){
    let ownership_string = String::from("Hello World");
}

fn ownership(s: String){
    println!("Owned string: {}", s);
}