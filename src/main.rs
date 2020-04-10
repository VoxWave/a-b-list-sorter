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
use std::collections::{HashMap, VecDeque};
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
    sort(&mut lines, |a, b| if a == b {
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

pub fn sort<T, F>(vec: &mut [T], mut cmp: F)
where F: FnMut(&T, &T) -> Ordering 
{
    println!("uhh");
    let runs = get_runs(vec, &mut cmp);
    println!("{:?}", runs);
    unify_order_of_runs(&runs, vec, &mut cmp);
    merge(runs, vec, &mut cmp);
}

fn get_runs<T, F>(vec: &mut [T], mut cmp: &mut F) -> VecDeque<(usize, usize)>
where F: FnMut(&T, &T) -> Ordering
{
    let mut runs = VecDeque::with_capacity(vec.len());
    let mut current_run = (0,1);
    let mut current_order = None;
    for i in 0..vec.len() {
        let this = &vec[i];
        match vec.get(i+1) {
            Some(next) => {
                match current_order {
                    Some(Ordering::Equal) | None => {
                        current_order = Some(cmp(this, next));
                        current_run.1 = i+2;
                    },
                    Some(order) => {
                        match (order, cmp(this, next)) {
                            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => {
                                //new run begins since there was an ordering change.
                                current_run.1 = i+1;
                                runs.push_back(current_run);
                                current_order = Some(cmp(this, next));
                                current_run = (i+1, i+2);
                            },
                            _ => current_run.1 = i+2,
                        }
                    },
                }
            },
            None => current_run.1 = i+1,
        }
    }
    runs.push_back(current_run);
    runs
}

fn unify_order_of_runs<T, F>(runs: &VecDeque<(usize, usize)>, vec: &mut [T], mut cmp: &mut F) 
where F: FnMut(&T, &T) -> Ordering 
{

}

fn merge<T, F>(runs: VecDeque<(usize, usize)>, vec: &mut [T], mut cmp: &mut F) 
where F: FnMut(&T, &T) -> Ordering 
{

}
