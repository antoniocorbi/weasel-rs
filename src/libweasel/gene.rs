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

use delegate::delegate;
use std::ops::{Deref, DerefMut};

use crate::libweasel::charset;
use std::fmt;

pub type GeneList<T> = Vec<Box<T>>;

// -- Classes: ------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Gene {
    data: char,
}

#[derive(Clone, Debug)]
pub struct MutableGene(Gene);

// -- Traits: -------------------------------------------------------------
pub trait GeneExt {
    fn get(&self) -> char;
    fn set(&mut self, c: char);
    fn set_random_data(&mut self);
}

pub trait MutableGeneExt {
    fn mutate_data(&mut self, mr: f64);
}

pub trait GeneCreationExt {
    fn new(c: char) -> Self;
    fn new_from_random() -> Self;
}

// -- Impl. blocks: -------------------------------------------------------

impl DerefMut for Gene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Deref for Gene {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
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
        //let oldd = self.get();
        self.data = charset::rand_char();
        // println!("srdd: old:{oldd}, new:{}", self.data);
    }
}

impl GeneExt for MutableGene {
    delegate! {
        to self.0 {
          fn get(&self) -> char;
          fn set(&mut self, c: char);
          fn set_random_data(&mut self);
        }
    }
}

impl GeneCreationExt for MutableGene {
    fn new(c: char) -> Self {
        MutableGene(Gene { data: c })
    }

    fn new_from_random() -> Self {
        let data = charset::rand_char();
        MutableGene(Gene { data })
    }
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribimos en el formateador 'f' la representación que queremos
        write!(f, "Gene: {} ", self.data)
    }
}

impl fmt::Display for MutableGene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribimos en el formateador 'f' la representación que queremos
        write!(f, "MutableGene: {} ", self.get())
    }
}

impl MutableGeneExt for MutableGene {
    fn mutate_data(&mut self, mr: f64) {
        use rand::Rng;
        let mut rng = rand::rng();
        // let p = rng.random_range(0.0..=1.0);
        let p: f64 = rng.random();

        if p < mr {
            //println!("mutate data: p: {p} mr: {mr}");
            self.set_random_data();
        }
    }
}

impl From<&Gene> for char {
    fn from(g: &Gene) -> Self {
        g.get()
    }
}

impl From<&MutableGene> for char {
    fn from(g: &MutableGene) -> Self {
        g.get()
    }
}

// -- Tests: --------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_gene() {
        let mut g = MutableGene::new('a');
        g.mutate_data(0.8);
        let c: char = (&g).into();
        assert!(c != 'a' || g.get() == 'a');
    }

    #[test]
    fn test_gene_deref() {
        let g = Gene::new('a');
        let c: char = *g;

        assert_eq!(c, 'a');
    }

    #[test]
    fn test_gene_derefmut() {
        let mut g = Gene::new('a');
        *g = 'z';
        let c: char = *g;

        assert_eq!(c, 'z');
    }
}
