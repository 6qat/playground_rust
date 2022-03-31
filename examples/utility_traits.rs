#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]

#[derive(Debug)]
struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("# Dropping {:?} #", self);
    }
}

fn main() {
    let _a1 = Appellation{
        name: "Guiga".to_string(),
        nicknames: vec!["a".to_string(), "b".to_string()]};
}