/*  a-b list sorter
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
use std::collections::HashMap;
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
    // Sort the list by asking the user a-b questions.
    let mut memoi = HashMap::new();
    bubble_sort(&mut lines, |a, b| if a == b {
            Ordering::Equal
        } else {
            memoi.get(&(b.clone(), a.clone()))
                .copied()
                .map(Ordering::reverse)
                .unwrap_or_else(|| *memoi.entry((a.clone(), b.clone()))
                    .or_insert_with(|| {
                        println!("a: {} b: {}", a, b);
                        loop {
                            let mut buf = String::with_capacity(1);
                            stdin().read_line(&mut buf).unwrap();
                            match buf.trim() {
                                "a" => return Ordering::Greater,
                                "b" => return Ordering::Less,
                                _ => println!("type a or b."),
                            };
                        }
                    }))
        }
    );
    // Print out the ordered list.
    println!("The final order is:\n");
    for line in lines {
        println!("{}", line);
    }
}

fn bubble_sort<T, F>(vec: &mut Vec<T>, mut cmp: F)
where F: FnMut(&T, &T) -> Ordering 
{
    for end in (0..vec.len()).rev() {
        let mut sorted = true;
        for i in 0..end {
            match cmp(&vec[i], &vec[i+1]) {
                Ordering::Less => {
                    vec.swap(i, i+1);
                    sorted = false;
                },
                _ => {},
            }
        }
        if sorted {
            break;
        }
    }
}
