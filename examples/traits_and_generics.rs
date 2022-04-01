use std::any::TypeId;
use std::io::Write;
use std::fs::File;
use std::io::Result;

fn _say_hello(out: &mut dyn Write) -> Result<()> {
    // if TypeId::of::<*mut dyn Write>() == TypeId::of::<File>() {
    //     println!("File");
    // }
    // match out {
    //     f@std::fs::File{inner} => {},
    //     _ => {}
    // }
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn _say_hello2<W: 'static + Write>(_out: &mut W) -> Result<()> {
    if TypeId::of::<W>() == TypeId::of::<File>() {
        println!("File");
    }
    // match _out {
    //     &mut File{inner} => { },
    //     _ => {},
    // }
    _out.write_all(b"hello world\n")?;
    _out.flush()
}

fn main() {
    let mut local_file = File::create("hello.txt").unwrap();
    _say_hello2(&mut local_file).unwrap();

    let mut bytes = vec![];
    _say_hello2(&mut bytes).unwrap();
    assert_eq!(bytes, b"hello world\n");
}