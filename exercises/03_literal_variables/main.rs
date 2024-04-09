////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    ($metavar1:literal plus $metavar2:literal) => {
        $metavar1 + $metavar2
    };
    (square $metavar:literal) => {
        $metavar * $metavar
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
