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

use weasel_rs::libweasel::{
    charset,
    gene::{Gene, GeneCreationExt, GeneExt},
};

use weasel_rs::libweasel::chromosome::StandardChromosome;

fn main() {
    let mut g = Gene::new('a');
    println!("Hello weasel with gene('a'): {}", g.get());
    g.set_random_data();
    println!("Hello weasel with gene(_random_): {}", g.get());

    for _ in 1..=32 {
        println!("rand. char: {}", charset::rand_char());
    }

    println!("ç is allowed char? {}", charset::in_char_set('ç'));
    println!("! is allowed char? {}", charset::in_char_set('!'));
    println!("@ is allowed char? {}", charset::in_char_set('@'));

    println!("_ is allowed char? {}", charset::in_char_set('_'));
    println!("9 is allowed char? {}", charset::in_char_set('9'));

    print!("Chromosome: ");
    let c = StandardChromosome::new("hola".into(), 4);
    println!("{c}");
}
