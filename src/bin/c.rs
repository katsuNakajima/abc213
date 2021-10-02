use std::vec;

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (h, w, n) = parse_line!(usize, usize, usize);
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();
    for _ in 0..n {
        let (x, y) = parse_line!(u32, u32);
        a.push(x);
        b.push(y);
    }
    for i in 1..n + 1 {
        let num = i as u32;
        if !a.contains(&num) {
            for j in i..n {
                a[j] -= 1;
            }
        }
        if !b.contains(&num) {
            for j in i..n {
                b[j] -= 1;
            }
        }
    }
    for i in 0..n {
        println!("{} {}", a[i], b[i]);
    }
}
