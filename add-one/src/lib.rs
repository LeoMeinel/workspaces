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
