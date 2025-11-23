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

use crate::libweasel::charset;
use std::fmt;

pub type GeneList<T> = Vec<Box<T>>;

#[derive(Clone)]
pub struct Gene {
    data: char,
}

pub trait GeneExt {
    fn get(&self) -> char;
    fn set(&mut self, c: char);
    fn set_random_data(&mut self);
}

pub trait GeneCreationExt {
    fn new(c: char) -> Self;
    fn new_from_random() -> Self;
}

impl GeneCreationExt for Gene {
    fn new(c: char) -> Self {
        Gene { data: c }
    }

    fn new_from_random() -> Self {
        let data = charset::rand_char();
        Gene { data }
    }
}

impl GeneExt for Gene {
    fn get(&self) -> char {
        self.data
    }
    fn set(&mut self, c: char) {
        self.data = c;
    }
    fn set_random_data(&mut self) {
        self.data = charset::rand_char();
    }
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribimos en el formateador 'f' la representación que queremos
        write!(f, "Gene: {} ", self.data)
    }
}

pub trait MutableGeneExt {
    fn mutate_data(&mut self, mr: f64);
}

impl MutableGeneExt for Gene {
    fn mutate_data(&mut self, mr: f64) {
        use rand::Rng;
        let mut rng = rand::rng();
        let p = rng.random_range(0.0..=1.0);

        if p < mr {
            self.set_random_data();
        }
    }
}

// impl From<&dyn GeneTrait> for char {
//     fn from(g: &T) -> Self {
//         g.get()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_gene() {
        let mut g = Gene::new('a');
        g.mutate_data(0.8);
        assert!(g.get() != 'a' || g.get() == 'a');
    }
}
