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

use crate::libweasel::gene::{Gene, GeneList};
use delegate::delegate;
use signals2::*;
use std::ops::Index;

// pub type GeneList = Vec<Box<Gene>>;

#[derive(Clone)]
struct ChromosomeData {
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

#[derive(Clone)]
pub struct Chromosome {
    // -- Data members: -------------------------------------------------------
    cd: ChromosomeData,
}

// -- Methods: ------------------------------------------------------------
impl ChromosomeData {
    // -- Methods: ------------------------------------------------------------
    pub fn new(tstr: String, ncopies: u32) -> Self {
        let mut cd = ChromosomeData {
            on_evolve_iteration: None,
            target_string: tstr,
            ncopies,
            gene_list: vec![],
        };
        cd.create_random_genes();

        cd
    }

    pub fn ncopies(&self) -> u32 {
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

impl Index<usize> for ChromosomeData {
    type Output = Box<Gene>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.gene_list[idx]
    }
}

impl Drop for ChromosomeData {
    fn drop(&mut self) {
        self.free_gene_list();
    }
}

impl Index<usize> for Chromosome {
    type Output = Box<Gene>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.cd[idx]
    }
}

// use std::ops::{Deref, DerefMut};
//
// impl Deref for Chromosome {
//     type Target = ChromosomeData; // El tipo interno al que se delega
//
//     fn deref(&self) -> &Self::Target {
//         &self.cd // Devuelve una referencia inmutable al campo interno
//     }
// }
//
// impl DerefMut for Chromosome {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.cd // Devuelve una referencia mutable al campo interno
//     }
// }

impl Chromosome {
    // -- Methods: ------------------------------------------------------------
    pub fn new(tstr: String, ncopies: u32) -> Self {
        let cd = ChromosomeData::new(tstr, ncopies);
        Chromosome { cd }
    }

    // Usamos el atributo 'delegate' para indicar que los siguientes
    // métodos deben delegar en el campo 'cd'.

    delegate! {
        to self.cd {
          // Se delegan estos métodos específicos
          pub fn ncopies(&self) -> u32;
          pub fn target(&self) -> String;
          // Se delega este método mutable
          fn create_genes_from_target(&mut self);
          fn create_random_genes(&mut self);
          pub fn size(&self) -> usize;
          pub fn fitness(&self, v: &GeneList) -> u32;
        }
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
