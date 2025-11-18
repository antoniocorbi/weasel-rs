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

use std::fmt;

pub struct Gene {
    data: char,
}

impl Gene {
    pub fn new(c: char) -> Self {
        Gene { data: c }
    }

    pub fn get(&self) -> char {
        self.data
    }
    pub fn set(&mut self, c: char) {
        self.data = c;
    }
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribimos en el formateador 'f' la representación que queremos
        write!(f, "Gene: {} ", self.data)
    }
}
