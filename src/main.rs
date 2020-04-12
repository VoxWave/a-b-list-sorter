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

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::io::stdin;
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
    sort(&mut lines, |a, b| {
        if a == b {
            Ordering::Equal
        } else {
            memoi
                .get(&(b.clone(), a.clone()))
                .copied()
                .map(Ordering::reverse)
                .unwrap_or_else(|| {
                    *memoi.entry((a.clone(), b.clone())).or_insert_with(|| {
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
                    })
                })
        }
    });
    // Print out the ordered list.
    println!("The final order is:\n");
    for line in lines {
        println!("{}", line);
    }
}

pub fn sort<T, F>(vec: &mut [T], mut cmp: F)
where
    F: FnMut(&T, &T) -> Ordering,
    T: std::fmt::Debug,
{
    // Sweep through the slice and locate runs of data that are in some order.
    let runs = get_runs(vec, &mut cmp);
    // Some runs are in the opposite order so reverse them.
    unify_order_of_runs(&runs, vec, &mut cmp);
    // Now we have runs of sorted data in the slice which we can merge together into one
    // run of ordered data I.E. sort the slice.
    merge(runs, vec, &mut cmp);
}

fn get_runs<T, F>(vec: &mut [T], cmp: &mut F) -> VecDeque<(usize, usize)>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut runs = VecDeque::with_capacity(vec.len());
    let mut current_run = (0, 1);
    let mut current_order = None;
    for i in 0..vec.len() {
        let this = &vec[i];
        match vec.get(i + 1) {
            Some(next) => {
                match current_order {
                    Some(Ordering::Equal) | None => {
                        current_order = Some(cmp(this, next));
                        current_run.1 += 1;
                    }
                    Some(order) => {
                        match (order, cmp(this, next)) {
                            (Ordering::Greater, Ordering::Less)
                            | (Ordering::Less, Ordering::Greater) => {
                                //new run begins since there was an ordering change.
                                runs.push_back(current_run);
                                current_order = None;
                                current_run = (i + 1, i + 2);
                            }
                            _ => current_run.1 += 1,
                        }
                    }
                }
            }
            None => runs.push_back(current_run),
        }
    }
    runs
}

fn unify_order_of_runs<T, F>(
    runs: &VecDeque<(usize, usize)>,
    vec: &mut [T],
    cmp: &mut F,
) 
where
    F: FnMut(&T, &T) -> Ordering,
{
    for (start, end) in runs {
        if end - 1 == *start {
            continue;
        }
        match cmp(&vec[*start], &vec[*start+1]) {
            Ordering::Less => {
                vec[*start..*end].reverse();
            },
            _ => {},
        }
    }
}

fn merge<T, F>(mut runs: VecDeque<(usize, usize)>, vec: &mut [T], cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{   
    while runs.len() > 1 {
        let mut new_runs = VecDeque::with_capacity(runs.len()/2+1);
        loop {
            match (runs.pop_front(), runs.pop_front()) {
                (Some(left), Some(right)) => new_runs.push_back(merge_adjacent(left, right, vec, cmp)),
                (Some(run), None) => {
                    new_runs.push_back(run);
                    break;
                },
                (None, None) => break,
                _ => unreachable!(),
            }
        }
        runs = new_runs;
    }
}

fn merge_adjacent<T, F>(left: (usize, usize), right: (usize, usize), vec: &mut [T], cmp: &mut F) -> (usize, usize) 
where
    F: FnMut(&T, &T) -> Ordering,
{
    assert!(left.1 == right.0);
    let mut left_top= left.0;
    let (mut right_top, right_bottom) = right;
    let mut sort_destination = left_top;
    loop{
        match cmp(&vec[left_top], &vec[right_top]) {
            Ordering::Greater | Ordering::Equal => {
                vec.swap(left_top, sort_destination);
                match (left_top + 1 == right_top, left_top == sort_destination) {
                    (true, true) => {
                        left_top += 1;
                        break;
                    },
                    (true, false) => {
                        sort_destination += 1;
                        left_top = sort_destination;
                    },
                    _ => {
                        left_top += 1;
                        sort_destination += 1;
                    },
                }
            },
            Ordering::Less => {
                vec.swap(right_top, sort_destination);
                if left_top == sort_destination {
                    left_top = right_top;
                }
                right_top += 1;
                sort_destination += 1;
                if right_top == right_bottom {
                    break;
                }
            }
        }
    }
    while left_top < right_top {
        let mut cur = left_top;
        while cur != sort_destination {
            vec.swap(cur, cur-1);
            cur -= 1;
        }
        left_top += 1;
    }
    (left.0, right.1)
}
// 2 6 10 11 | 3 5 7 9
// ^1          ^2
// 2 6 10 11 | 3 5 7 9
// ^s^1        ^2
// 2 3 10 11 | 6 5 7 9
//   ^s        ^1^2
// 2 3 5 11 | 6 10 7 9
//     ^s     ^1   ^2
// 2 3 5 6  |11 10 7 9
//       ^s     ^1 ^2
// 2 3 5 6  | 7 10 11 9
//            ^s ^1   ^2
// 2 3 5 6  | 7  9 11 10
//               ^s    ^1  ^2

// 4 3 2 1 | 6 5
// ^1s       ^2
// 6 3 2 1 | 4 5
//   ^s      ^1^2
// 6 5 3 2 | 1 4 
//     ^s      ^1   ^2

