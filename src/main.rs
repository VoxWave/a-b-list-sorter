use std::io::stdin;
use std::cmp::Ordering;
fn main() {
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
    println!("The final order is:\n");
    for line in lines {
        println!("{}", line);
    }
}
