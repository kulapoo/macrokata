////////// DO NOT CHANGE BELOW HERE /////////
fn print_vec<V: std::fmt::Debug>(vec: &Vec<V>) {
    println!("{vec:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `graph!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////

macro_rules! graph {
    (
        $(
            $key: literal->  (
                $($val: expr),* // "literal" much simpler
            );
        )
        *
    ) => {

        {
            let mut vec = Vec::new();
            $(
                $(
                    vec.push((
                        $key,
                        $val
                    ));
                )*
            )*

            vec
        }

    };
}

#[allow(clippy::vec_init_then_push)]
fn main() {
    let my_graph = graph!(
        1 -> (2, 3, 4, 5);
        2 -> (1, 3);
        3 -> (2);
        4 -> ();
        5 -> (1, 2, 3);
    );

    print_vec(&my_graph);
}
