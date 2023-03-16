g2p::g2p!(GF2, 1);

pub fn byte_into_gf2(b: u8) -> [GF2; 8]
{
    [GF2(b % 2),
    GF2(b / 2 % 2),
    GF2(b / 4 % 2),
    GF2(b / 8 % 2),
    GF2(b / 16 % 2),
    GF2(b / 32 % 2),
    GF2(b / 64 % 2),
    GF2(b / 128 % 2)]
}

pub fn gf2_into_byte(s: &[GF2; 8]) -> u8
{
    s[0].0 +
    s[1].0 * 2 +
    s[2].0 * 4 +
    s[3].0 * 8 +
    s[4].0 * 16 +
    s[5].0 * 32 +
    s[6].0 * 64 +
    s[7].0 * 128
}

#[cfg(test)]
mod tests {
    use super::{gf2_into_byte, byte_into_gf2};


    #[test]
    fn into_and_back() {
        for i in 0..=255{
            let i = i as u8;
            let s = byte_into_gf2(i);
            let f = gf2_into_byte(&s);
            assert_eq!(f, i);
        }
    }
}