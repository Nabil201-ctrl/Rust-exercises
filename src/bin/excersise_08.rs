// Find the largest number in an array manually.


fn main(){
    let arr = [1, 2, 3, 4, 55, 78, 3, 65];
    let maximum = arr.iter().max().unwrap();
    println!("the maximum of the array is ${}", maximum);
    return();
}
