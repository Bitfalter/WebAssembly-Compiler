pub fn from_u32(value: u32) -> Vec<u8> {
    fn encode(i: u32, r: &[u8]) -> Vec<u8> {
        let b = i & 0x7fu32;
        let ii = i >> 7;
        if ii == 0 {
            [r, &[b as u8]].concat()
        } else {
            let r = [r, &[(0x80u32 | b) as u8]].concat();
            encode(ii, &r)
        }
    }
    encode(value, &[]).to_vec()
}
