pub struct SST {
    bytes: Vec<u8>,
}

impl SST {
    pub fn keys_size(&self) -> u32 {
        self.extract_u32(0)
    }

    fn key_offset(&self, key_index: u32) -> u32 {
        self.extract_u32(4 + key_index * 4)
    }

    fn key(&self, key_index: u32) -> &[u8] {
        let offset = self.key_offset(key_index);
        self.extract_string(offset)
    }

    fn key_size(&self, key_index: u32) -> u32 {
        let offset = self.key_offset(key_index);
        self.extract_u32(offset)
    }


    fn get(&self, key: &Vec<u8>) -> Option<&[u8]> {
        for i in 0..self.keys_size() {
            let candidate_key = self.key(i);
            if candidate_key.eq(key) {
                return Some(self.value(i));
            }
        }
        None
    }

    fn value_offset(&self, value_index: u32) -> u32 {
        let key_offset = self.key_offset(value_index);
        let key_size = self.key_size(value_index);
        self.extract_u32(key_offset + key_size + 4)
    }

    fn value(&self, value_index: u32) -> &[u8] {
        let offset = self.value_offset(value_index);
        self.extract_string(offset)
    }

    fn extract_u32(&self, offset: u32) -> u32 {
        let offset = offset as usize;
        u32::from_be_bytes([
            self.bytes[offset],
            self.bytes[offset + 1],
            self.bytes[offset + 2],
            self.bytes[offset + 3],
        ])
    }

    fn extract_string(&self, offset: u32) -> &[u8] {
        let string_len = self.extract_u32(offset) as usize;
        let start = (offset + 4) as usize;
        let end = start + string_len;
        &self.bytes[start..end]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_operations() {
        let bytes = vec!(
            // Offsets
            0,0,0,1,    /* number offsets */
            0,0,0,8,    /* first offset */
            // Key
            0,0,0,1,    /* size */
            2,          /* key */
            0,0,0,17,   /* offset of value */
            // Value
            0,0,0,1,    /* size */
            3           /* value */
        );

        let sst = SST { bytes };
        assert_eq!(sst.keys_size(), 1);
        assert_eq!(sst.key_offset(0), 8);
        assert_eq!(sst.key_size(0), 1);
        assert_eq!(sst.key(0), vec!(2));
        assert_eq!(sst.value_offset(0), 17);
        assert_eq!(sst.value(0), vec!(3));
        let result: [u8; 1] = [3];
        assert_eq!(sst.get(&vec!(2)), Some(&result[0..1]));
    }
}
