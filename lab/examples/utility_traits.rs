#[allow(unused_assignments, unused_variables, dead_code)]
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
    let _a0: Appellation;
    {
        let _a1 = Appellation {
            name: "Guiga".to_string(),
            nicknames: vec!["a".to_string(), "b".to_string()],
        };
        //_a0 = _a1;
    }

    let _a1 = Appellation {
        name: "Chattiness".to_string(),
        nicknames: vec!["c".to_string(), "d".to_string()],
    };

    drop(_a1);

    println!("Before the program ends.")
}
