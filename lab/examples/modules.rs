use random_info::RandomInfo;

mod random_info;

// use crate::random_info::RandomInfo;

#[allow(dead_code)]
struct MyData {
    my_id: i32,
    random: RandomInfo,
}

impl RandomInfo {
    pub fn is_larger(
        &self,
        compare_to: i32,
    ) -> bool {
        self.id > compare_to
    }
}

fn main() {
    let _ri = RandomInfo {
        call_count: 0,
        id: 10,
    };
    let _md = MyData {
        my_id: 20,
        random: RandomInfo::new(),
    };

    let _smaller = _ri.is_smaller(10);
    let _larger = _ri.is_larger(10);
}
