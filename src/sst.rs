struct SST {
    bytes: Vec<u8>,
}

impl SST {
    fn key_size(&self) -> u32 {
        parse_u32(0)
    }

    fn parse_u32(&self, usize: offset) -> u32 {
        
    }
}

fn parse()

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut sst = SST { bytes: vec!(0,0,0,6) };
        assert_eq!(sst.key_size(), 6);
    }

}
