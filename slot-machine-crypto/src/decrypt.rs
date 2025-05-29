pub fn decrypt(x: &[u8], k: &[u8]) -> Vec<u8> {
    let k = crate::common::rk(k);

    let mut x = x.to_vec();

    for x in x.chunks_exact_mut(32) {
        let mut r = u128::from_le_bytes(x[..16].try_into().unwrap());
        let mut l = u128::from_le_bytes(x[16..].try_into().unwrap());

        for i in 0..19 {
            l ^= crate::common::f(r, k[18 - i]);
            (l, r) = (r, l)
        }

        x[..16].copy_from_slice(&r.to_le_bytes());
        x[16..].copy_from_slice(&l.to_le_bytes());
    }

    u(&mut x);
    x
}

// Unpad the input to 32 bytes using PKCS7
pub fn u(x: &mut Vec<u8>) {
    let p = x.last().unwrap();
    x.resize(x.len() - (*p as usize), 0);
}
