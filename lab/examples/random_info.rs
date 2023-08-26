#[allow(dead_code)]
pub struct RandomInfo {
    pub call_count: i64,
    pub id: i32,
}

impl RandomInfo {
    pub fn new() -> Self {
        Self {
            call_count: 0,
            id: 0,
        }
    }
    pub fn is_smaller(&self, compare_to: i32) -> bool {
        self.id < compare_to
    }
}

// https://rust-lang.github.io/rust-clippy/master/index.html#/new_without_default
impl Default for RandomInfo {
    fn default() -> Self {
        RandomInfo::new()
    }
}

#[allow(dead_code)]
fn main() {}
