// https://planetmath.org/goodhashtableprimes

// const HASHTABLE_SIZE: usize = 2_147_483_647;
const HASHTABLE_SIZE: usize = 100_663_319;

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
        self.keys[hash % HASHTABLE_SIZE].hash = hash;
        self.keys[hash % HASHTABLE_SIZE].value = value;
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
