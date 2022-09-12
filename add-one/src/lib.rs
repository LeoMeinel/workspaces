/*
 * File: lib.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * cargo run -p add-one to run
 */

use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn gen_random_number_external() -> i32 {
    rand::thread_rng().gen_range(0..100)
}

/*
 * cargo test -p add-one to run
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn is_in_correct_range() {
        assert!(gen_random_number_external() <= 99 && gen_random_number_external() >= 0)
    }
}
