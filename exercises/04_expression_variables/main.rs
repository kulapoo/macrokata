////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////
macro_rules! math {
    ($lhs: expr, plus, $rhs: expr) => {
        $lhs + $rhs
    };
    (square $arg: expr) => {
        $arg * $arg
    }
}


fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
}
