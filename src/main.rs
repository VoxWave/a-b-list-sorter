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
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Write};

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let args = App::new("a-b-list-sorter")
        .arg(
            Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .help("Specifies file from which to be sorted data is read from."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .help("Specifies file to which the sorted data is written to."),
        )
        .arg(
            Arg::with_name("state")
                .short("s")
                .takes_value(true)
                .help("Load partial sorting state from specified file."),
        )
        .get_matches();
    let mut lines = if let Some(filename) = args.value_of("input") {
        // Read the list of items from file.
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<_, _>>()?
    } else {
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
        lines
    };
    // Sort the list by asking the user a-b questions.
    let mut memoi = HashMap::new();
    if let Some(filename) = args.value_of("state") {
        println!("Loading sorting state from `{}`", filename);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for (n, line) in reader
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .enumerate()
        {
            let mut split = line.split("|");
            let a = split
                .next()
                .ok_or(format!("No a on line {}", n))?
                .to_owned();
            let ord = split.next().ok_or(format!("No ordering on line {}", n))?;
            let b = split
                .next()
                .ok_or(format!("No b on line {}", n))?
                .to_owned();
            match ord {
                "<" => {
                    memoi.insert((a, b), Ordering::Less);
                }
                ">" => {
                    memoi.insert((a, b), Ordering::Greater);
                }
                _ => panic!("Unknown ordering `{}` on line {}", ord, n),
            }
        }
    }

    println!("You can save the sorting state by inputting `save <filename>`");
    merge_sort(&mut lines, |a, b| {
        if a == b {
            Ordering::Equal
        } else {
            memoi
                .get(&(b.clone(), a.clone()))
                .copied()
                .map(Ordering::reverse)
                .unwrap_or_else(|| {
                    memoi
                        .get(&(a.clone(), b.clone()))
                        .copied()
                        .unwrap_or_else(|| {
                            let result = ask_user(a, b, &memoi);
                            memoi.insert((a.clone(), b.clone()), result);
                            result
                        })
                })
        }
    });
    if let Some(filename) = args.value_of("output") {
        let mut file = File::create(filename)?;
        for line in lines {
            file.write(line.as_bytes())?;
            file.write(b"\n")?;
        }
        println!("Saved sorted list to {}", filename);
    } else {
        // Print out the ordered list.
        println!("The final order is:\n");
        for line in lines {
            println!("{}", line);
        }
    }
    Ok(())
}

fn ask_user(a: &str, b: &str, memoi: &HashMap<(String, String), Ordering>) -> Ordering {
    println!("a: {}\n\nb: {}", a, b);
    loop {
        let mut buf = String::with_capacity(1);
        stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "a" => return Ordering::Greater,
            "b" => return Ordering::Less,
            s => {
                const SAVE: &str = "save";
                if s.starts_with(SAVE) {
                    let filename = s[SAVE.len()..].trim();
                    let mut file = File::create(filename).unwrap();
                    for ((a, b), ord) in memoi {
                        file.write(a.as_bytes()).unwrap();
                        file.write(match ord {
                            Ordering::Less => b"|<|",
                            Ordering::Equal => panic!(),
                            Ordering::Greater => b"|>|",
                        })
                        .unwrap();
                        file.write(b.as_bytes()).unwrap();
                        file.write(b"\n").unwrap();
                    }
                    println!("Saved current sorting state to {}", filename);
                }
                println!("type a or b.")
            }
        };
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

pub fn merge_sort<T, F>(vec: &mut [T], mut cmp: F)
where
    F: FnMut(&T, &T) -> Ordering,
    T: std::fmt::Debug,
{
    let runs = get_merge_sort_runs(vec, &mut cmp);
    unify_order_of_runs(&runs, vec, &mut cmp);
    merge(runs, vec, &mut cmp);
}

fn get_merge_sort_runs<T, F>(vec: &mut [T], _: &mut F) -> VecDeque<(usize, usize)>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut runs = VecDeque::with_capacity(vec.len());
    for i in (0..vec.len()).step_by(2) {
        if i + 2 > vec.len() {
            runs.push_back((i, i + 1));
        } else {
            runs.push_back((i, i + 2));
        }
    }
    runs
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

fn unify_order_of_runs<T, F>(runs: &VecDeque<(usize, usize)>, vec: &mut [T], cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    for (start, end) in runs {
        if end - 1 == *start {
            continue;
        }
        match cmp(&vec[*start], &vec[*start + 1]) {
            Ordering::Less => {
                vec[*start..*end].reverse();
            }
            _ => {}
        }
    }
}

fn merge<T, F>(mut runs: VecDeque<(usize, usize)>, vec: &mut [T], cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    while runs.len() > 1 {
        let mut new_runs = VecDeque::with_capacity(runs.len() / 2 + 1);
        loop {
            match (runs.pop_front(), runs.pop_front()) {
                (Some(left), Some(right)) => {
                    new_runs.push_back(merge_adjacent(left, right, vec, cmp))
                }
                (Some(run), None) => {
                    new_runs.push_back(run);
                    break;
                }
                (None, None) => break,
                _ => unreachable!(),
            }
        }
        runs = new_runs;
    }
}

fn merge_adjacent<T, F>(
    left: (usize, usize),
    right: (usize, usize),
    vec: &mut [T],
    cmp: &mut F,
) -> (usize, usize)
where
    F: FnMut(&T, &T) -> Ordering,
{
    assert!(left.1 == right.0);
    let mut left_top = left.0;
    let (mut right_top, right_bottom) = right;
    loop {
        match cmp(&vec[left_top], &vec[right_top]) {
            Ordering::Greater | Ordering::Equal => {
                left_top += 1;
                if left_top == right_top {
                    break;
                };
            }
            Ordering::Less => {
                let mut this = right_top;
                while this != left_top {
                    vec.swap(this, this - 1);
                    this -= 1;
                }
                left_top += 1;
                right_top += 1;
                if right_top == right_bottom {
                    break;
                }
            }
        }
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
#[test]
fn sort_test_1() {
    let mut original_list = vec![2, 6, 10, 11, 3, 5, 7, 9];
    let mut cloned_list = original_list.clone();
    sort(&mut original_list, |a, b| b.cmp(a));
    cloned_list.sort();
    assert_eq!(original_list, cloned_list);
}

// 4 3 2 1 | 6 5
// ^1s       ^2
// 6 3 2 1 | 4 5
//   ^s      ^1^2
// 6 5 3 2 | 1 4
//     ^s      ^1   ^2
#[test]
fn sort_test_2() {
    let mut original_list = vec![4, 3, 2, 1, 6, 5];
    let mut cloned_list = original_list.clone();
    sort(&mut original_list, |a, b| a.cmp(b));
    cloned_list.sort_by(|a, b| b.cmp(a));
    assert_eq!(original_list, cloned_list);
}

#[test]
fn sort_test_3() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        sort(&mut sorted, |a, b| b.cmp(a));
        assert_eq!(vec, sorted);
    }
}

#[test]
fn merge_sort_test_1() {
    let mut original_list = vec![2, 6, 10, 11, 3, 5, 7, 9];
    let mut cloned_list = original_list.clone();
    merge_sort(&mut original_list, |a, b| b.cmp(a));
    cloned_list.sort();
    assert_eq!(original_list, cloned_list);
}

#[test]
fn merge_sort_test_2() {
    let mut original_list = vec![4, 3, 2, 1, 6, 5];
    let mut cloned_list = original_list.clone();
    merge_sort(&mut original_list, |a, b| a.cmp(b));
    cloned_list.sort_by(|a, b| b.cmp(a));
    assert_eq!(original_list, cloned_list);
}

#[test]
fn merge_sort_test_3() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        merge_sort(&mut sorted, |a, b| b.cmp(a));
        assert_eq!(vec, sorted);
    }
}

#[test]
fn runs_should_be_the_same() {
    let mut vec = vec![1, 2, 1, 2, 1, 2, 1, 2];
    let merge_runs = get_merge_sort_runs(&mut vec, &mut (|_, _| Ordering::Equal));
    let other_runs = get_runs(&mut vec, &mut (|a, b| a.cmp(b)));
    println!("merge {:?}\nnonmerge {:?}", merge_runs, other_runs);
    assert_eq!(merge_runs, other_runs);
}

#[test]
fn merge_is_better() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(500);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    let mut sort_cmps = 0;
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        sort(&mut sorted, |a, b| {
            sort_cmps += 1;
            b.cmp(a)
        });
        assert_eq!(vec, sorted);
    }
    println!("{}", sort_cmps);

    let mut merge_sort_cmps = 0;
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        merge_sort(&mut sorted, |a, b| {
            merge_sort_cmps += 1;
            b.cmp(a)
        });
        assert_eq!(vec, sorted);
    }
    println!("{}", merge_sort_cmps);

    assert!(merge_sort_cmps < sort_cmps);
}

#[test]
fn merge_is_better_2() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    let mut merge_score = 0;
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        let mut sorted2 = sorted.clone();
        let mut merge_cmps = 0;
        merge_sort(&mut sorted, |a, b| {
            merge_cmps += 1;
            b.cmp(a)
        });
        let mut merge_runs_cmps = 0;
        sort(&mut sorted2, |a, b| {
            merge_runs_cmps += 1;
            b.cmp(a)
        });
        println!("merge: {}, merge_runs: {}", merge_cmps, merge_runs_cmps);
        if merge_cmps < merge_runs_cmps {
            merge_score += 1;
        } else {
            merge_score -= 1;
        }
    }
    assert!(merge_score == 10000);
}

#[test]
fn merge_is_better_than_std() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    let mut merge_score = 0;
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        let mut sorted2 = sorted.clone();
        let mut merge_cmps = 0;
        merge_sort(&mut sorted, |a, b| {
            merge_cmps += 1;
            b.cmp(a)
        });
        let mut std_cmps = 0;
        sorted2.sort_by(|a, b| {
            std_cmps += 1;
            b.cmp(a)
        });
        println!("merge: {}, std: {}", merge_cmps, std_cmps);
        if merge_cmps < std_cmps {
            merge_score += 1;
        } else {
            merge_score -= 1;
        }
    }
    assert!(merge_score == 10000);
}

#[test]
fn merge_is_better_than_std_unstable() {
    use rand::seq::SliceRandom;
    let mut vec = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        vec.push(i);
    }
    let mut merge_score = 0;
    for _ in 0..10000 {
        let mut sorted = vec.clone();
        &mut sorted.shuffle(&mut rng);
        let mut sorted2 = sorted.clone();
        let mut merge_cmps = 0;
        merge_sort(&mut sorted, |a, b| {
            merge_cmps += 1;
            b.cmp(a)
        });
        let mut std_cmps = 0;
        sorted2.sort_unstable_by(|a, b| {
            std_cmps += 1;
            b.cmp(a)
        });
        println!("merge: {}, std: {}", merge_cmps, std_cmps);
        if merge_cmps < std_cmps {
            merge_score += 1;
        } else {
            merge_score -= 1;
        }
    }
    assert!(merge_score == 10000);
}
