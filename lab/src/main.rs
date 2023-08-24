#![allow(unused_assignments, dead_code, unused_variables)]
fn main() {
    let a = [55, 66, 77];
    let mut v = vec![55, 66, 77, 99];

    let s1 = &a[0..2];
    let s2 = &v[0..2];

    v.push(10);
}
