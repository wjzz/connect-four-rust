// https://planetmath.org/goodhashtableprimes

// const HASHTABLE_SIZE: usize = 2_147_483_647;  // 32 gigs
// const HASHTABLE_SIZE: usize = 1_610_612_741;  // 25 gigs

const HASHTABLE_SIZE: usize = 100_663_319; // 1.5 gigs
                                           // const HASHTABLE_SIZE: usize = 50_331_653;
                                           // const HASHTABLE_SIZE: usize = 25_165_843;
                                           // const HASHTABLE_SIZE: usize = 12_582_917;
                                           // const HASHTABLE_SIZE: usize = 6_291_469;
                                           // const HASHTABLE_SIZE: usize = 3_145_739;
                                           // const HASHTABLE_SIZE: usize = 1_572_869;

#[derive(Clone)]
struct Entry {
    lower_hash: u32,
    big_hash: u32,
    value: usize,
}

pub struct Table {
    keys: Vec<Entry>,
}

const UNKNOWN: usize = 5;

// TODO: we can encode the hash value as well?
// value:
// 6: big work
// 3: big value
// 3: lower value

impl Table {
    pub fn new() -> Table {
        let empty = Entry {
            lower_hash: 0,
            big_hash: 0,
            value: UNKNOWN,
        };
        let keys = vec![empty.clone(); HASHTABLE_SIZE];
        Table { keys }
    }

    pub fn insert(self: &mut Self, hash: usize, value: usize, work: usize) {
        assert!(value <= 4);

        let field = &mut self.keys[hash % HASHTABLE_SIZE];

        let field_work = field.value >> 6;
        if work > field_work {
            let lower_value = field.value & 0b111;
            field.value = lower_value | (value << 3) | (work << 6);
            field.big_hash = hash as u32;
        } else {
            field.value = value | ((field.value >> 3) << 3);
            field.lower_hash = hash as u32;
        }
    }

    pub fn get(self: &mut Self, hash: usize) -> Option<usize> {
        let field = &self.keys[hash % HASHTABLE_SIZE];
        if field.value != UNKNOWN {
            let hash = hash as u32;
            if field.big_hash == hash {
                let big_value = (field.value >> 3) & 0b111;
                return Some(big_value);
            } else if field.lower_hash == hash {
                let lower_value = field.value & 0b111;
                assert!(lower_value <= 4);
                return Some(lower_value);
            }
        }
        return None;
    }
}
