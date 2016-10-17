pub mod point;
use point::*;

#[macro_use]
extern crate text_io;

fn input() -> Vec<Point> {
    let n: usize = read!();
    let mut p: Vec<Point> = Vec::<Point>::with_capacity(n);
    for _ in 0..n {
        let x: f64 = read!();
        let y: f64 = read!();
        p.push(Point{x: x, y: y});
    }
    return p;
}

fn output(p: &[Point]) {
    let n = p.len();
    println!("{}", n);
    for i in 0..n {
        println!("{}", p[i].to_string());
    }
}

fn find_ind<T, F>(a: &[T], f: F) -> Option<usize>
    where F: Fn(&T, &T) -> bool
{
    let n = a.len();

    if a.len() == 0 {
        return Option::None;
    }

    let mut res = 0;
    for i in 1..n {
        if f(&a[i], &a[res]) {
            res = i;
        }
    }

    return Option::Some(res);
}

fn add_f(stack: &mut Vec<Point>, z: &Point) {
    if stack.len() == 0 {
        panic!("Empty stack!");
    }

    while stack.len() >= 2 {
        let sn = stack.len();
        let a = stack[sn - 2].clone();
        let b = stack[sn - 1].clone();

        let ab = Point::vector(&a, &b);
        let bc = Point::vector(&b, &z);

        if Point::cross_prod(&ab, &bc) > 0. {
            stack.push(z.clone());
            break;
        }

        stack.pop();
    }

    if stack.len() < 2 {
        stack.push(z.clone());
    }
}

fn solve(mut p: Vec<Point>) -> Vec<Point> {
    let n = p.len();

    let ind = find_ind(&p, |a: &Point, b: &Point| {a.y < b.y}).unwrap();
    p.swap(0, ind);

    let o = p[0].clone();

    (&mut p[1..]).sort_by(|a: &Point, b: &Point| {
        let oa = Point::vector(&o, a);
        let ob = Point::vector(&o, b);
        match oa.arg().partial_cmp(&ob.arg()) {
            Option::Some(ord) => ord,
            Option::None      => panic!("WTF happened?"),
        }
    });

    let mut stack: Vec<Point> = Vec::<Point>::with_capacity(n);
    stack.push(o.clone());

    for i in 1..n {
        add_f(&mut stack, &p[i].clone());
    }
    add_f(&mut stack, &o.clone());

    stack.pop();

    stack
}

fn main() {
    let p: Vec<Point> = input();
    let q: Vec<Point> = solve(p);
    output(&q);
}
