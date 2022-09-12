/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * cargo run -p adder to run
 */

use rand::Rng;

use add_one::{add_one, gen_random_number_external};
use add_two::add_two;

fn main() {
    print_numbers();
}

fn print_numbers() {
    let rand_num_0 = gen_random_number_internal();
    let rand_num_1 = gen_random_number_external();
    println!(
        "Random number 0: {} + 1 = {}",
        rand_num_0,
        add_one(rand_num_0)
    );
    println!(
        "Random number 1: {} + 2 = {}",
        rand_num_1,
        add_two(rand_num_1)
    );
}

fn gen_random_number_internal() -> i32 {
    rand::thread_rng().gen_range(0..6)
}

/*
 * cargo test -p adder to run
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_in_correct_range() {
        assert!(gen_random_number_internal() <= 5 && gen_random_number_internal() >= 0)
    }
}
