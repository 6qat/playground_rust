#![allow(unused, dead_code, unused_variables)]

use std::iter::once;

use itertools::{
    all, any, assert_equal, chain, cloned, concat, enumerate, fold,
    intersperse_with, iproduct, iterate, join, kmerge, merge, multiunzip,
    multizip, partition, repeat_n, Itertools,
};

fn main() {}

#[test]
fn test_interleave() {
    for elt in itertools::interleave(&[1, 2, 3], &[2, 3, 4]) { // loop body
    }

    let it = (1..3).interleave(vec![-1, -2]);
    assert_equal(it, vec![1, -1, 2, -2]);
}

#[test]
fn test_intersperse() {
    let it =
        itertools::Itertools::intersperse((1..5), 15).collect::<Vec<_>>();
    // let it = (1 .. 5).intersperse(15).collect::<Vec<_>>();
    assert_equal(it, vec![1, 15, 2, 15, 3, 15, 4]);
}

#[test]
fn test_intersperse_with() {
    let mut i = 10;

    assert_equal(
        intersperse_with((0..3), || {
            i -= 1;
            i
        }),
        vec![0, 9, 1, 8, 2],
    );

    assert_eq!(i, 8);
}

#[test]
fn test_iproduct() {
    // Iterate over the coordinates of a 4 x 4 x 4 grid
    // from (0, 0, 0), (0, 0, 1), .., (0, 1, 0), (0, 1, 1), .. etc until (3, 3, 3)
    for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
        // ..
        println!("({}, {}, {})", i, j, k);
    }
}

#[test]
fn test_chain() {
    let with_macro = chain!((2..6), repeat_n(1, 3), once(30));
    // for i in with_macro.clone() {
    //     println!("->> i: {}", i);
    // }
    // for i in &with_macro.collect::<Vec<_>>() {
    //     println!("->> i: {}", i);
    // }

    let with_method = (2..6).chain(repeat_n(1, 3)).chain(once(30));
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

#[test]
fn test_enumerate() {
    for (i, elt) in enumerate(&[10, 20, 30]) {
        // loop body
        println!("->> i: {}, elt: {}", i, elt);
    }
}

#[test]
fn test_equal() {
    assert!(itertools::equal(vec![1, 2, 3], (1..4)));
    assert!(!itertools::equal(&[0, 0], &[0, 0, 0]));
}

#[test]
fn test_fold() {
    assert_eq!(fold(&[1., 2., 3.], 0., |a, &b| f32::max(a, b)), 3.);
}

#[test]
fn test_iterate() {
    assert_equal(iterate(1, |&i| i * 3).take(5), vec![1, 3, 9, 27, 81]);
}

#[test]
fn test_join() {
    assert_eq!(join(&[1, 2, 3], ", "), "1, 2, 3");
}

#[test]
fn test_kmerge() {
    assert_eq!(
        kmerge(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]])
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    // assert_eq!(concat(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).iter().collect::<Vec<_>>().sort(),
    //            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]

fn test_merge() {
    assert_eq!(
        merge(vec![1, 2, 3], vec![2, 3, 4]).collect::<Vec<_>>(),
        vec![1, 2, 2, 3, 3, 4]
    );
}

#[test]
fn test_multiunzip() {
    let inputs = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];

    let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(inputs);

    assert_eq!(a, vec![1, 4, 7]);
    assert_eq!(b, vec![2, 5, 8]);
    assert_eq!(c, vec![3, 6, 9]);
}

#[test]
fn test_multizip() {
    // iterate over three sequences side-by-side
    let mut results = [0, 0, 0, 0];
    let inputs = [3, 7, 9, 6];

    for (r, index, input) in multizip((&mut results, 0..10, &inputs)) {
        *r = index * 10 + input;
    }

    assert_eq!(results, [0 + 3, 10 + 7, 29, 36]);
}

#[test]
fn test_partition() {
    let mut data = [7, 1, 1, 7, 1, 1, 7];
    let split_index = partition(&mut data, |elt| *elt >= 3);

    assert_eq!(data, [7, 7, 7, 1, 1, 1, 1]);
    assert_eq!(split_index, 3);
}

#[test]
fn test_repeat_call() {
    use itertools::repeat_call;
    use itertools::Itertools;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::from(vec![2, 5, 3, 7, 8]);

    // extract each element in sorted order
    for element in repeat_call(|| heap.pop()).while_some() {
        print!("{}", element);
    }

    assert_equal(repeat_call(|| 1).take(5), vec![1, 1, 1, 1, 1]);
}
