#![allow(unused_assignments, dead_code, unused_variables)]
fn main() {
    let a = [55, 66, 77];
    let mut v1 = vec![55, 66, 77, 99];

    let s1 = &a[0..2];
    let s2 = &v1[0..2];

    v1.push(10);

    let v_iter = v1.iter();
    for i in v_iter {
        println!("->> i: {}", i);
    }

    let v_iter_mut = v1.iter_mut(); // The variable itself is NOT immutable.
    for i in v_iter_mut {
        println!("->> mut i: {}", i);
    }

    let v_iter_owned = v1.into_iter();
    for i in v_iter_owned {
        println!("->> owned i: {}", i);
    }

    // for i in v_iter_owned {
    //     println!("->> owned i: {}", i);
    // }
}

#[test]
fn iterator_demonstration() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter(); // NEEDS TO BE MUTABLE!!!! (look above comment for iter_mut())
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![55, 66, 77, 99];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 297);
}
