////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////
macro_rules! math {
    ($a:literal plus $b:literal) => {
        $a + $b
    };
    (square $a:literal) => {
        $a * $a
    };
}


fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
