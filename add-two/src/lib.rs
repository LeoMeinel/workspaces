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
 * cargo run -p add-two to run
 */
pub fn add_two(x: i32) -> i32 {
    x + 2
}

/*
 * cargo test -p add-two to run
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(4, add_two(2));
    }
}
