// Make a simple calculator using match.

fn main(){
    let num1 = 10;
    let num2 = 20;
    let operator = '+';
}

fn calculator(num1:i32, num2: i32, operator: char){
    match operator {
        '+' => {
            num1 + num2;
        },
        '-' => {
            num1 - num2;

            return;
        },
        '*' => {
            num1 * nu m2;
        }
        _ => {
            return(print!("input a proper number"))
        },

    }
}