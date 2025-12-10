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

use std::fmt::Arguments;

use colored::Colorize;
use signals2::*;
use weasel_rs::libweasel::{
    arguments, charset,
    chromosome::{EvolvingChromosome, StandardChromosome},
    gene::{Gene, GeneCreationExt, GeneExt},
};

fn check1() {
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

fn check_evolve() {
    let args = arguments::Arguments::from_app_args();
    let s = args.sentence().to_owned();
    let nc = args.ncopies();
    let mr = args.mr();
    let mut ec = EvolvingChromosome::new(s, nc).with_mr(mr);

    ec.on_evolve_iteration.connect(|it, bf, chromosome| {
        let size = chromosome.size();
        println!(
            // "On it.:{it} fitness is {bf} and mr: {}: {}",
            // chromosome.mr(),
            "{} ({bf}/{size}) @{it}",
            chromosome.get_genes_colored()
        );
        //println!("On it.:{it} fitness is {bf}");
    });

    ec.evolve();
}

fn check_colors() {
    let parte1 = "¡Hola".yellow().bold();
    let parte2 = " Mundo!".cyan().italic();
    let resultado_format = format!("{}{}", parte1, parte2) + " > FIN.";
    println!("Resultado format: {resultado_format}");

    let mut cs = "this is red on blue ❤🧡💛💚💙💜".white().on_bright_red();
    println!("{cs}");
    println!("{}", "you can also make bold comments".bold());
    println!("{}", "this is blue".blue());
    println!(
        "{}",
        "you can specify color by string"
            .color("black")
            .on_color("yellow")
    );
    println!(
        "{}",
        format!("{} {} !", "it".green(), "works".blue().bold())
    );
}

fn check_args() {
    let args = arguments::Arguments::from_app_args();
    println!("Sentence: {}", args.sentence());
    println!("Mutation rate: {}", args.mr());
    println!("Number of copies: {}", args.ncopies());
    println!("Encoded output?: {}", args.encoded());
}

fn main() {
    // check1();
    // check_colors();
    // check_args();
    check_evolve();
}
