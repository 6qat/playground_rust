use std::io::Write;
use std::fs::File;
use std::io::Result;

fn say_hello(out: &mut dyn Write) -> Result<()>  {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn main() {
    let mut local_file = File::create("hello.txt").unwrap();
    say_hello(&mut local_file).unwrap();

    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap();
    assert_eq!(bytes, b"hello world\n");

}