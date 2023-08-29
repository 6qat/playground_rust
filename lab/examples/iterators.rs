#![allow(unused_assignments, dead_code, unused_variables)]

use std::fmt::Display;

// https://www.youtube.com/watch?v=4GcKrj4By8k&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=20
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

    // The iterator itself is NOT immutable, but the vector it points to is.
    let v_iter_mut = v1.iter_mut();
    for i in v_iter_mut {
        println!("->> mut i: {}", i);
    }

    // Steals the ownership of the values from v1.
    let v_iter_owned = v1.into_iter();
    for i in v_iter_owned {
        println!("->> owned i: {}", i);
    }

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl Display for Number {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "Number: {}", self.value)
        }
    }

    let a = vec![
        Number { value: 55 },
        Number { value: 66 },
        Number { value: 77 },
    ];

    // into_iter() under the hood.
    for i in a {
        println!("->> i: {}", i);
    }
    // println!("{:?}", a);
    // let _ = a.iter();
    // let _ = v1.iter();

    use std::collections::VecDeque;

    let vec = [1, 2, 3, 4];
    let buf = vec.into_iter().collect::<VecDeque<_>>();
    // let _ = vec.iter();

    let mut vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![10, 20, 30, 40];
    vec1.extend(vec2);

    let vec = vec![1, 2, 3];
    for x in vec.iter().rev() {
        println!("vec contained {x:?}");
    }
}

#[test]
fn iterator_demonstration() {
    let v = vec![1, 2, 3];

    // NEEDS TO BE MUTABLE!!!! (look above comment for iter_mut())
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![55, 66, 77, 99];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // Necessary to specify the type of total.
    assert_eq!(total, 297);
}

#[test]
fn iterator_map() {
    let v1 = vec![55, 66, 77, 99];
    let v1_iter = v1.iter();

    // Map is returns a lazy iterator. So cool!!!!
    let v2 = v1_iter.map(|x| x + 1);
    //let v22 = v2.collect();

    // Necessary to specify the type of v2 (the container, not the elements).
    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // Using "turbofish" syntax instead of a type annotation on the receiver.
    let v3 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();

    assert_eq!(v3, vec![56, 67, 78, 100]);

    let _ = v1.iter().collect::<Vec<_>>();
}

#[test]
fn iterator_to_string() {
    let chars = ['g', 'd', 'k', 'k', 'n'];

    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();

    let hello = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect::<String>();

    assert_eq!("hello", hello);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(
    shoes: Vec<Shoe>,
    shoe_size: u32,
) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associated type.
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
            // Some(self.count + 1)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    // The next() method is defined by the Iterator trait.
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum = Counter::new()
        .skip(1)
        .zip(Counter::new())
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum::<u32>();

    assert_eq!(18, sum);
}
