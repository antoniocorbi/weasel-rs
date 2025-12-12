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

use std::env;

pub struct Arguments {
    s: String,
    mr: f64,
    ncopies: u32,
    encoded: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            s: String::from("Me thinks it's like a weasel"),
            mr: 0.08,
            ncopies: 500,
            encoded: false,
        }
    }
}

impl Arguments {
    pub fn from_app_args() -> Self {
        let mut arguments = Self::default();
        // The first argument (index 0) is always the path used to execute the program.
        let args: Vec<String> = env::args().collect();
        let mut argsiter = args.iter();

        // 2. The total number of arguments
        let num_args = args.len();

        if num_args > 1 {
            for (i, a) in argsiter.enumerate() {
                match a.as_str() {
                    "-s" | "--sentence" => {
                        let s = args.get(i + 1);
                        arguments.set_sentence(s.unwrap());
                    }
                    "-m" | "--mrate" => {
                        let m = args.get(i + 1).unwrap();
                        let m = m.parse::<f64>().ok().unwrap();
                        arguments.set_mr(m);
                    }
                    "-n" | "--ncopies" => {
                        let nc = args.get(i + 1).unwrap();
                        let nc = nc.parse::<u32>().ok().unwrap();
                        arguments.set_ncopies(nc);
                    }
                    "-d" | "--encoded" => {
                        arguments.set_encoded(true);
                    }
                    &_ => (),
                }
            }
        }

        arguments
    }

    pub fn set_sentence(&mut self, s: &str) {
        self.s = String::from(s);
    }

    pub fn sentence(&self) -> &str {
        &self.s
    }

    pub fn mr(&self) -> f64 {
        self.mr
    }

    pub fn ncopies(&self) -> u32 {
        self.ncopies
    }

    pub fn encoded(&self) -> bool {
        self.encoded
    }

    pub fn set_mr(&mut self, mr: f64) {
        self.mr = mr;
    }

    pub fn set_ncopies(&mut self, nc: u32) {
        self.ncopies = nc;
    }

    pub fn set_encoded(&mut self, e: bool) {
        self.encoded = e
    }
}
