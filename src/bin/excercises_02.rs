// Swap two numbers without a temporary variable.

fn main (){

    swap(2, 7)
}


fn swap (a:i32, b:i32 ){
    let mut a = a;
    let mut b = b;
    a = b;
    a = a + b;
    b = a - b;
    a = a - b;

    }