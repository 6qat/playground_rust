#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }
    //assert_eq!(*r, 1);

    let v = vec![1, 2, 3];
    let r = &v[1];
    assert_eq!(*r, 2);

    static mut STASH: &i32 = &10;

    fn func(p: &'static i32) {
        unsafe {
            STASH = p;
        }

    }

    static WORTH_POINTING_AT: i32 = 1000;
    func(&WORTH_POINTING_AT);
    let us = unsafe { *STASH };
    assert_eq!(us, 1000);

}
