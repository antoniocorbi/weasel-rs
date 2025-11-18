// Copyright (C) 2025  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use once_cell::sync::Lazy;
use std::string::String;
// import commonly used items from the prelude:
use rand::prelude::*;

static CHARSET: Lazy<String> = Lazy::new(|| {
    let s = " ,.;:_-abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ0123456789";

    format!("{}", s)
});

pub fn in_char_set(c: char) -> bool {
    CHARSET.contains(c)
}

pub fn rand_char() -> char {
    let i = (rand::random::<u64>() as usize) % CHARSET.chars().count();
    let c = CHARSET.chars().nth(i);

    c.unwrap()
}
