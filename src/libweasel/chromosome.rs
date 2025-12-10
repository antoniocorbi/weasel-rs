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

use crate::libweasel::gene::{
    Gene, GeneCreationExt, GeneExt, GeneList, MutableGene, MutableGeneExt,
};
use colored::Colorize;
// use delegate::delegate;
use signals2::*;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

// pub type GeneList = Vec<Box<Gene>>;
pub type StandardChromosome = Chromosome<Gene>;
pub type EvolvingChromosome = Chromosome<MutableGene>;
pub trait ChromosomeExt: GeneCreationExt + GeneExt + Clone + 'static {}

#[derive(Clone)]
pub struct Chromosome<T: ChromosomeExt> {
    // -- Data members: -------------------------------------------------------
    /// The signal to emit; (it, best_fit, current_chromosome)
    pub on_evolve_iteration: Signal<(u32, u32, Rc<Self>)>,
    /// Our target string
    target_string: String,
    /// Number of copies in each evolution
    ncopies: u32,
    /// The gene list of this chromosome
    gene_list: GeneList<T>,
    /// Mutation rate
    mr: f64,
}

// -- Impl. blocks: -------------------------------------------------------
impl ChromosomeExt for Gene {}
impl ChromosomeExt for MutableGene {}

impl<T: ChromosomeExt> fmt::Display for Chromosome<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut gstr = String::new();
        // Escribimos en el formateador 'f' la representación que queremos
        self.gene_list.iter().for_each(|e| {
            let c = <T as GeneExt>::get(e);
            gstr.push(c);
        });
        write!(f, "{}", gstr)
    }
}

impl Chromosome<MutableGene> {
    pub fn with_mr(mut self, mr: f64) -> Self {
        self.mr = mr;
        self
    }

    fn mutate_genes(&self, v: &mut GeneList<MutableGene>) {
        for i in 0..self.size() {
            //let c = Box::new(self[i].clone());
            let c = &self[i];

            v[i].set(c.get());
            v[i].mutate_data(self.mr());
        }
    }

    pub fn mr(&self) -> f64 {
        self.mr
    }

    pub fn evolve(&mut self) {
        let mut it: u32 = 0;
        let mut glc: GeneList<MutableGene> = vec![]; // Gene list copy
        let mut bgl: GeneList<MutableGene> = vec![]; // Best Gene list copy

        self.gene_list.iter().for_each(|e| {
            let mg1 = Box::new(MutableGene::new(e.get()));
            let mg2 = Box::new(MutableGene::new(e.get()));
            glc.push(mg1);
            bgl.push(mg2);
        });

        // println!("glc: {:?}", glc);

        // Best fit til now.
        let mut bf: u32 = self.fitness(&glc);

        loop {
            it += 1;
            for _ in 0..self.ncopies() {
                self.mutate_genes(&mut glc);
                let f = self.fitness(&glc);

                // println!("Loop: {it} - f: {f} - bf: {bf} - {:#?}", bgl);
                // if it % 100 == 0 {
                //     println!("Loop: {it} - f: {f} - bf: {bf}: {}", self.get_genes());
                // }

                if f < bf {
                    bf = f;

                    for i in 0..self.size() {
                        bgl[i].set(glc[i].get());
                    }

                    // println!(
                    //     "it: {it} - bf: {bf} - bgl: {:?}",
                    //     Chromosome::gene_list_as_string(&bgl)
                    // );

                    if bf == 0 {
                        // bestfit == 0 means the chromosome is equal to target-string.
                        // println!("Found: {}", bf);
                        break;
                    }
                }
            }

            self.gene_list.iter_mut().enumerate().for_each(|(i, g)| {
                g.set(bgl[i].get());
            });

            // Emit the signal
            let self_rc = Rc::new(self.clone());
            self.on_evolve_iteration.emit(it, bf, self_rc.clone());

            //check if we've got the target string: bf (best fit) == 0.
            if bf == 0 {
                break;
            }
        }
    }
}

impl<T: ChromosomeExt> Chromosome<T> {
    // -- Methods: ------------------------------------------------------------
    pub fn new(tstr: String, ncopies: u32) -> Self {
        let on_evolve_iteration = Signal::new();
        let mut c = Chromosome {
            on_evolve_iteration,
            target_string: tstr,
            ncopies,
            gene_list: vec![],
            mr: 0.0,
        };
        c.create_random_genes();

        c
    }

    pub fn ncopies(&self) -> u32 {
        self.ncopies
    }

    pub fn target(&self) -> String {
        self.target_string.clone()
    }

    #[allow(unused)]
    fn create_genes_from_target(&mut self) {
        self.free_gene_list();
        // for c in self.target_string.chars() {
        //     self.gene_list.push(Box::new(T::new(c)));
        // }

        self.gene_list = self
            .target_string
            .chars()
            .map(|c| Box::new(T::new(c)))
            .collect();
    }

    #[allow(unused)]
    fn gene_list_as_string(gene_list: &GeneList<T>) -> String {
        let mut gstr = String::new();

        gene_list.iter().for_each(|e| {
            let c = <T as GeneExt>::get(e);
            gstr.push(c);
        });

        gstr
    }

    pub fn get_genes(&self) -> String {
        Self::gene_list_as_string(&self.gene_list)
    }

    /// Mark wrong genes with a different color.
    pub fn get_genes_colored(&self) -> String {
        let gs = Self::gene_list_as_string(&self.gene_list);
        let ts = self.target();
        let mut coloredstr = "".to_owned();

        // 1. Get the character iterators for both strings.
        let chars_gs = gs.chars();
        let chars_ts = ts.chars();

        chars_gs.zip(chars_ts).for_each(|(g, t)| {
            let mut charstr: String;
            charstr = format!("{}", g);
            if g != t {
                // Char in gene sequence is different from the one in target string
                charstr = format!("{}", charstr.red());
            }
            coloredstr += &charstr;
        });

        coloredstr
    }

    fn create_random_genes(&mut self) {
        self.free_gene_list();
        for _ in 0..self.target_string.len() {
            self.gene_list.push(Box::new(T::new_from_random()));
        }
    }

    fn free_gene_list(&mut self) {
        self.gene_list.clear();
    }

    pub fn size(&self) -> usize {
        self.gene_list.len()
    }

    pub fn fitness(&self, v: &GeneList<T>) -> u32 {
        let mut d: u32 = 0;
        let mut i = 0;

        for c in self.target_string.chars() {
            if c != (&*v[i]).get() {
                d += 1;
            }
            i += 1;
        }

        d
    }
}

impl<T: ChromosomeExt> Index<usize> for Chromosome<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &*self.gene_list[idx]
    }
}

impl<T: ChromosomeExt> IndexMut<usize> for Chromosome<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut *self.gene_list[idx]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_genes() {
        let c = StandardChromosome::new("hola".into(), 4);
        let gstr = c.get_genes();

        assert_eq!(gstr.len(), "hola".len());
    }

    #[test]
    fn test_evolvingchromosome1() {
        let mut c = EvolvingChromosome::new("hola".into(), 4);
        c.create_genes_from_target();
        let gc0 = (c[0]).get();
        let gc1 = (c[1]).get();

        assert_eq!(gc0, 'h');
        assert_eq!(gc1, 'o');
    }

    #[test]
    fn test_evolvingchromosome2() {
        let mut c = EvolvingChromosome::new("hola".into(), 4);
        c.create_genes_from_target();
        c[0] = MutableGene::new('l');
        let gc0 = c[0].get();

        assert_eq!(gc0, 'l');
    }

    #[test]
    fn test_index1() {
        let mut c = StandardChromosome::new("hola".into(), 4);
        c.create_genes_from_target();
        let gc0: char = (&c[0]).into();
        let gc1 = c[1].get();

        assert_eq!(gc0, 'h');
        assert_eq!(gc1, 'o');
    }

    #[test]
    fn test_fitness1() {
        let c = StandardChromosome::new("hola".into(), 4);
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
        let c = StandardChromosome::new("hola".into(), 4);
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
        let c = StandardChromosome::new("hola".into(), 4);
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
