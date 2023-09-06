#![allow(unused, dead_code, unused_variables)]

use std::iter::once;

use itertools::{
    all,
    any,
    assert_equal,
    chain,
    cloned,
    concat,
    iproduct,
    repeat_n,
    Itertools,
};

fn main() {}

#[test]
fn test_interleave() {
    for elt in itertools::interleave(&[1, 2, 3], &[2, 3, 4]) { // loop body
    }

    let it = (1 .. 3).interleave(vec![-1, -2]);
    assert_equal(it, vec![1, -1, 2, -2]);
}

#[test]
fn test_intersperse() {
    let it =
        itertools::Itertools::intersperse((1 .. 5), 15).collect::<Vec<_>>();
    // let it = (1 .. 5).intersperse(15).collect::<Vec<_>>();
    assert_equal(it, vec![1, 15, 2, 15, 3, 15, 4]);
}

#[test]
fn test_iproduct() {
    // Iterate over the coordinates of a 4 x 4 x 4 grid
    // from (0, 0, 0), (0, 0, 1), .., (0, 1, 0), (0, 1, 1), .. etc until (3, 3, 3)
    for (i, j, k) in iproduct!(0 .. 4, 0 .. 4, 0 .. 4) {
        // ..
        println!("({}, {}, {})", i, j, k);
    }
}

#[test]
fn test_chain() {
    let with_macro = chain!((2 .. 6), repeat_n(1, 3), once(30));
    // for i in with_macro.clone() {
    //     println!("->> i: {}", i);
    // }
    // for i in &with_macro.collect::<Vec<_>>() {
    //     println!("->> i: {}", i);
    // }

    let with_method = (2 .. 6).chain(repeat_n(1, 3)).chain(once(30));
    assert_equal(with_macro, with_method);

    let mut result: Vec<i32> = Vec::new();

    for element in chain(&[1, 2, 3], &[4]) {
        result.push(*element);
    }
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_concat() {
    let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
    assert_eq!(concat(input), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_all() {
    assert!(all(&[1, 2, 3], |elt| *elt > 0));
}

#[test]
fn test_any() {
    assert!(any(&[0, -1, 2], |elt| *elt > 0));
}

#[test]
fn test_cloned() {
    assert_eq!(cloned(b"abc").next(), Some(b'a'));
}
