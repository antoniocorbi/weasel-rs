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
        c.create_genes();

        c
    }

    pub fn target(&self) -> String {
        self.target_string.clone()
    }

    fn create_genes(&mut self) {
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

impl Drop for Chromosome {
    fn drop(&mut self) {
        self.free_gene_list();
    }
}
