// https://planetmath.org/goodhashtableprimes

// const HASHTABLE_SIZE: usize = 2_147_483_647;
const HASHTABLE_SIZE: usize = 100_663_319;

#[derive(Clone)]
struct Entry {
    hash: usize,
    value: usize,
}

pub struct Table {
    keys: Vec<Entry>,
    pub collissions: usize,
}

const UNKNOWN: usize = 5;

impl Table {
    pub fn new() -> Table {
        let empty = Entry { hash: 0, value: UNKNOWN };
        let keys = vec![empty.clone(); HASHTABLE_SIZE];
        Table { keys, collissions: 0 }
    }

    pub fn insert(self: &mut Self, hash: usize, value: usize, work: usize) {
        assert!(value <= 4);

        let field = &mut self.keys[hash % HASHTABLE_SIZE];
        if field.value != UNKNOWN {
            self.collissions += 1;
        }
        // let field_work = field.value >> 3;
        field.hash = hash;
        field.value = value | (work << 3);
    }

    pub fn get(self: &Self, hash: usize) -> Option<usize> {
        let field = &self.keys[hash % HASHTABLE_SIZE];
        if field.hash == hash && field.value != UNKNOWN {
            return Some(field.value & 0b111);
        } else {
            return None;
        }
    }
}
