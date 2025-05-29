pub fn encrypt(x: &[u8], k: &[u8]) -> Vec<u8> {
    let k = crate::common::rk(k);

    let mut x = p(x);

    for x in x.chunks_exact_mut(32) {
        let mut l = u128::from_le_bytes(x[..16].try_into().unwrap());
        let mut r = u128::from_le_bytes(x[16..].try_into().unwrap());

        for i in 0..19 {
            l ^= crate::common::f(r, k[i]);
            (l, r) = (r, l)
        }

        x[..16].copy_from_slice(&l.to_le_bytes());
        x[16..].copy_from_slice(&r.to_le_bytes());
    }

    x
}

// Pad the input to 32 bytes using PKCS7
pub fn p(x: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; (x.len() / 32 + 1) * 32];

    v[..x.len()].copy_from_slice(x);

    let p = (v.len() - x.len()) as u8;

    for i in 0..p {
        v[x.len() + (i as usize)] = p;
    }

    v
}
