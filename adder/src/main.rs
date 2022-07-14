/*
 * workspaces is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/workspaces/blob/main/LICENSE
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
