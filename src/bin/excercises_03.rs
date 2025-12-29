// Even or odd checker -> Use % operator and if/else.

fn main() {
    fn checker(a: i32) {
        if a % 2 == 0 {
            println!("{} is an Even Number", a);
        } else {
            println!("{} is an Odd Number", a);
        }
    }

    checker(10);
    checker(7);
}