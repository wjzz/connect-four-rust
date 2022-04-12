// https://planetmath.org/goodhashtableprimes

const HASHTABLE_SIZE: usize = 2_147_483_647;  // 32 gigs
// const HASHTABLE_SIZE: usize = 1_610_612_741;  // 25 gigs

// const HASHTABLE_SIZE: usize = 2_147_483_647;
// const HASHTABLE_SIZE: usize = 100_663_319;
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
    pub collissions: usize,
    pub lowers: usize,
    pub uppers: usize,
    pub inserts: usize,
    pub gets: usize,
    pub get_misses: usize,
}

const UNKNOWN: usize = 5;

// TODO: we can encode the hash value as well?
// value:
// 5: big work
// 3: big value
// 3: lower value

impl Table {
    pub fn new() -> Table {
        let empty = Entry { lower_hash: 0, big_hash: 0, value: UNKNOWN };
        let keys = vec![empty.clone(); HASHTABLE_SIZE];
        Table { keys, collissions: 0, uppers: 0, lowers: 0, inserts: 0, gets: 0, get_misses: 0 }
    }

    pub fn insert(self: &mut Self, hash: usize, value: usize, work: usize) {
        assert!(value <= 4);
        assert!(work <= 31);

        self.inserts += 1;
        let field = &mut self.keys[hash % HASHTABLE_SIZE];
        if field.value & 0b111 != UNKNOWN {
            self.collissions += 1;
        }

        let field_work = field.value >> 6;
        if work > field_work {
            let lower_value = field.value & 0b111;
            field.value = lower_value | (value << 3) | (work << 6);
            field.big_hash = hash as u32;
            self.uppers += 1;
        } else {
            field.value = value | ((field.value >> 3) << 3);
            field.lower_hash = hash as u32;
            self.lowers += 1;
        }
    }

    pub fn get(self: &mut Self, hash: usize) -> Option<usize> {
        let field = &self.keys[hash % HASHTABLE_SIZE];
        self.gets += 1;
        if field.value != UNKNOWN {
            let hash = hash as u32;
            if field.big_hash == hash {
                let big_value = (field.value >> 3) & 0b111;
                return Some(big_value);
            } else if field.lower_hash == hash {
                return Some(field.value & 0b111);
            }
        }
        self.get_misses += 1;
        return None;
    }
}
