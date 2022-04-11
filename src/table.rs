// https://planetmath.org/goodhashtableprimes

const HASHTABLE_SIZE: usize = 2_147_483_647;  // 32 giga
// const HASHTABLE_SIZE: usize = 1_610_612_741;  // 25 giga

// 3.2
// 201326611

#[derive(Clone)]
struct Entry {
    hash: usize,
    value: i32,
}

pub struct Table {
    keys: Vec<Entry>,
    pub collissions: usize,
}

impl Table {
    pub fn new() -> Table {
        let empty = Entry { hash: 0, value: -1 };
        let keys = vec![empty.clone(); HASHTABLE_SIZE];
        Table { keys, collissions: 0 }
    }

    pub fn insert(self: &mut Self, hash: usize, value: i32) {
        assert!(value.abs() <= 10);

        let field = &mut self.keys[hash % HASHTABLE_SIZE];
        if field.value != -1 {
            self.collissions += 1;
        }
        field.hash = hash;
        field.value = value;
    }

    pub fn get(self: &Self, hash: usize) -> Option<i32> {
        let field = &self.keys[hash % HASHTABLE_SIZE];
        if field.hash == hash && field.value != -1 {
            return Some(field.value);
        } else {
            return None;
        }
    }
}
