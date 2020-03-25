/*a-b list sorter
    Copyright (C) 2020  Victor "VoxWave" Bankowski

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.*/

use std::io::stdin;
use std::cmp::Ordering;
fn main() {
    // Fetch the list items from the command line.
    let mut lines = Vec::new();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let buf: String = buf.trim().chars().collect();
        lines.push(buf);
    }
    lines.reverse();
    // Sort the list by asking the user a-b questions.
    lines.sort_by(|a, b| {
        println!("a: {} b: {}", a, b);
        loop {
            let mut buf = String::with_capacity(1);
            stdin().read_line(&mut buf).unwrap();
            match buf.trim() {
                "a" => return Ordering::Less,
                "b" => return Ordering::Greater,
                _ => println!("type a or b."),
            };
        }
    });
    // Print out the ordered list.
    println!("The final order is:\n");
    for line in lines {
        println!("{}", line);
    }
}
