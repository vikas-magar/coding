mod ae;
mod lc;

use ae::array_of_product;
use ae::first_duplicate_value;
use ae::longest_peak;
use ae::monotonic_array;
use ae::move_element_end;
use ae::smallest_diff;
use mylib::*;

use lc::astroid_collusion;

fn main() {
    smallest_diff::sd();
    move_element_end::run();
    monotonic_array::run();
    longest_peak::run();

    array_of_product::run();

    first_duplicate_value::run();
    astroid_collusion::run();
    println!(
        "{}",
        array::longest_range::run(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
    let m = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    for ele in m {
        println!("welcome to the india!")
    }
}
