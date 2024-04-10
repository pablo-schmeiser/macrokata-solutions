////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    ($expr1:expr, plus, $expr2:expr) => {
        $expr1 + $expr2
    };
    (square $my_expr:expr) => {
        $my_expr * $my_expr
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
}
