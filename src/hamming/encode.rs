//Используется код Хэмминга со следующим определением: 
// x1 + x2 + x3 = x5
// x1 + x3 + x4 = x6
// x2 + x3 + x4 = x7

use crate::types::GF2;

pub fn get_syndrome(w: &[GF2; 7]) -> [GF2; 3]
{
    [w[0] + w[1] + w[2] + w[4], 
    w[0] + w[2] + w[3] + w[5],
    w[1] + w[2] + w[3] + w[6]]
}

pub fn restore_message(w: &[GF2; 7]) -> [GF2; 7]
{
    let mut m = *w;
    match get_syndrome(w) {
        [GF2(0), GF2(0), GF2(0)] => _ = 1,
        [GF2(1), GF2(1), GF2(0)] => m[0] += 1.into(),
        [GF2(1), GF2(0), GF2(1)] => m[1] += 1.into(),
        [GF2(1), GF2(1), GF2(1)] => m[2] += 1.into(),
        [GF2(0), GF2(1), GF2(1)] => m[3] += 1.into(),
        [GF2(1), GF2(0), GF2(0)] => m[4] += 1.into(),
        [GF2(0), GF2(1), GF2(0)] => m[5] += 1.into(),
        [GF2(0), GF2(0), GF2(1)] => m[6] += 1.into(),
        _ => todo!()
    }
    m
}

pub fn encode_in(w: &[GF2; 7], msg: &[GF2; 3]) -> [GF2; 7]
{
    let mut m = restore_message(w);
    match msg {
        [GF2(0), GF2(0), GF2(0)] => _ = 1,
        [GF2(1), GF2(1), GF2(0)] => m[0] += 1.into(),
        [GF2(1), GF2(0), GF2(1)] => m[1] += 1.into(),
        [GF2(1), GF2(1), GF2(1)] => m[2] += 1.into(),
        [GF2(0), GF2(1), GF2(1)] => m[3] += 1.into(),
        [GF2(1), GF2(0), GF2(0)] => m[4] += 1.into(),
        [GF2(0), GF2(1), GF2(0)] => m[5] += 1.into(),
        [GF2(0), GF2(0), GF2(1)] => m[6] += 1.into(),
        _ => todo!()
    }
    m
}

#[cfg(test)]
mod tests {
    use crate::types::GF2;

    use super::{encode_in, get_syndrome};

    #[test]
    fn decode_encode() {
        for i in 0..8{
            let w = [GF2(0); 7];
            let msg = [GF2(i % 2), GF2(i / 2 % 2), GF2(i / 2 % 2)];
            let res = encode_in(&w, &msg);
            let res_msg = get_syndrome(&res);
            assert_eq!(msg, res_msg);
        }
    }
}