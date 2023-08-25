#![allow(unused_assignments, unused_variables, dead_code, unused)]

use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // unwrap() panics on error, while ? operator returns the error.

    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    Ok(())
}
fn _back() {
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
