struct SST {
    bytes: Vec<u8>,
}

impl SST {
    fn key_size(&self) -> u32 {
        self.extract_u32(0)
    }

    fn extract_u32(&self, offset: usize) -> u32 {
        u32::from_be_bytes([
            self.bytes[offset],
            self.bytes[offset + 1],
            self.bytes[offset + 2],
            self.bytes[offset + 3],
        ])
    }

    fn key_offset(&self, key_index: u32) -> u32 {
        self.extract_u32((4 + key_index * 4).try_into().unwrap())
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
        assert_eq!(sst.key_size(), 1);
        assert_eq!(sst.key_offset(0), 8);
    }
}
