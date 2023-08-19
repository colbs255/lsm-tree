use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct MemBlock {
    map: HashMap<Vec<u8>, Vec<u8>>,
    byte_count: usize,
}

impl MemBlock {

    pub fn new() -> MemBlock {
        MemBlock { map: HashMap::new(), byte_count: 0 }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.byte_count += key.len() + value.len();
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &Vec<u8>) -> Option<&Vec<u8>> {
        self.map.get(key)
    }

    pub fn size(&self) -> usize {
        self.byte_count
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut key_offsets: Vec<u32> = Vec::new();
        let mut keys: Vec<KeyCell> = Vec::new();
        let mut value_bytes: Vec<Vec<u8>> = Vec::new();

        let sorted_map: BTreeMap<_, _> = self.map.into_iter().collect();
        let count = sorted_map.len() as u32;
        let mut key_index = 4 * count;
        for (k, v) in sorted_map {
            key_offsets.push(key_index);
            let cell = KeyCell { key: k, value_offset: 0 };
            key_index += cell.serialized_size() as u32;
            keys.push(cell);
            value_bytes.push(v);
        }

        let mut value_index = key_index;
        for (i, value_b) in value_bytes.iter().enumerate() {
            keys[i].value_offset = value_index;
            let serialized_size = (value_b.len() + 4) as u32;
            value_index += serialized_size;
        }

        let mut result: Vec<u8> = Vec::new();
        // Offsets
        for offset in key_offsets {
            let bytes = u32_to_bytes(offset);
            result.extend_from_slice(&bytes);
        }

        // Keys
        for key_cell in keys {
            let bytes = u32_to_bytes(key_cell.key.len() as u32);
            result.extend_from_slice(&bytes);
            result.extend(&key_cell.key);

            let bytes = u32_to_bytes(key_cell.value_offset);
            result.extend_from_slice(&bytes);
        }

        // Values
        for v in value_bytes {
            let bytes = u32_to_bytes(v.len() as u32);
            result.extend_from_slice(&bytes);
            result.extend(&v);
        }

        result
    }
}

fn u32_to_bytes(value: u32) -> [u8; 4] {
    [
        ((value >> 24) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ]
}

struct KeyCell {
    key: Vec<u8>,
    value_offset: u32,
}

impl KeyCell {
    fn serialized_size(&self) -> usize {
        4 + 4 + self.key.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut block = MemBlock::new();
        assert_eq!(block.size(), 0);

        block.insert(vec!(1, 2, 3), vec!(1));
        assert_eq!(block.size(), 4);

        assert_eq!(block.get(&vec!(1,2,3)).unwrap(), &vec!(1));
    }

    #[test]
    fn serialize() {
        let mut block = MemBlock::new();
        block.insert(vec!(2), vec!(3));
        let result = block.serialize();
        let expected = vec!(
            // Offsets
            0,0,0,4,
            // Key
            0,0,0,1,    /* size */
            2,          /* key */
            0,0,0,13,   /* offset of value */
            // Value
            0,0,0,1,    /* size */
            3           /* value */
        );
        assert_eq!(result, expected);
    }
}
