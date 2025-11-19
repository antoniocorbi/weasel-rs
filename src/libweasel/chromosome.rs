// Copyright (C) 2025  Antonio-M. Corbi Bellot
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

use crate::libweasel::gene::Gene;
use signals2::*;
use std::ops::Index;

type GeneList = Vec<Box<Gene>>;

#[derive(Clone)]
struct Chromosome {
    // -- Data members: -------------------------------------------------------
    /// The signal to emit
    pub on_evolve_iteration: Option<Signal<(u32, u32, Box<Chromosome>)>>,
    /// Our target string
    target_string: String,
    /// Number of copies in each evolution
    ncopies: u32,
    /// The gene list of this chromosome
    gene_list: GeneList,
}

// -- Methods: ------------------------------------------------------------
impl Chromosome {
    // -- Methods: ------------------------------------------------------------
    pub fn new(tstr: String, ncopies: u32) -> Self {
        let mut c = Chromosome {
            on_evolve_iteration: None,
            target_string: tstr,
            ncopies,
            gene_list: vec![],
        };
        c.create_random_genes();

        c
    }

    pub fn ncopies(self) -> u32 {
        self.ncopies
    }

    pub fn target(&self) -> String {
        self.target_string.clone()
    }

    fn create_genes_from_target(&mut self) {
        self.free_gene_list();
        for c in self.target_string.chars() {
            self.gene_list.push(Box::new(Gene::new(c)));
        }
    }

    fn create_random_genes(&mut self) {
        self.free_gene_list();
        for _ in 0..self.target_string.len() {
            self.gene_list.push(Box::new(Gene::new_from_random()));
        }
    }

    fn free_gene_list(&mut self) {
        self.gene_list.clear();
    }

    pub fn size(&self) -> usize {
        self.gene_list.len()
    }

    pub fn fitness(&self, v: &GeneList) -> u32 {
        let mut d: u32 = 0;
        let mut i = 0;

        for c in self.target_string.chars() {
            if c != (&*v[i]).into() {
                d += 1;
            }
            i += 1;
        }

        d
    }
}

impl Index<usize> for Chromosome {
    type Output = Box<Gene>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.gene_list[idx]
    }
}

impl Drop for Chromosome {
    fn drop(&mut self) {
        self.free_gene_list();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index1() {
        let mut c = Chromosome::new("hola".into(), 4);
        c.create_genes_from_target();
        let gc0 = <&Gene as Into<char>>::into(&*c[0]);
        let gc1: char = (&*c[1]).into();

        assert_eq!(gc0, 'h');
        assert_eq!(gc1, 'o');
    }

    #[test]
    fn test_fitness1() {
        let c = Chromosome::new("hola".into(), 4);
        let g1 = Box::new(Gene::new('h'));
        let g2 = Box::new(Gene::new('a'));
        let g3 = Box::new(Gene::new('l'));
        let g4 = Box::new(Gene::new('a'));
        let mut v = vec![];
        v.push(g1);
        v.push(g2);
        v.push(g3);
        v.push(g4);

        assert_eq!(c.fitness(&v), 1);
    }

    #[test]
    fn test_fitness2() {
        let c = Chromosome::new("hola".into(), 4);
        let g1 = Box::new(Gene::new('h'));
        let g2 = Box::new(Gene::new('a'));
        let g3 = Box::new(Gene::new('l'));
        let g4 = Box::new(Gene::new('o'));
        let mut v = vec![];
        v.push(g1);
        v.push(g2);
        v.push(g3);
        v.push(g4);

        assert_eq!(c.fitness(&v), 2);
    }

    #[test]
    fn test_fitness3() {
        let c = Chromosome::new("hola".into(), 4);
        let g1 = Box::new(Gene::new('h'));
        let g2 = Box::new(Gene::new('o'));
        let g3 = Box::new(Gene::new('l'));
        let g4 = Box::new(Gene::new('a'));
        let mut v = vec![];
        v.push(g1);
        v.push(g2);
        v.push(g3);
        v.push(g4);

        assert_eq!(c.fitness(&v), 0);
    }
}
