// GF modulo
const P: u128 = 0x713486320eae95bc889dfda6543f4958;

/// Galois field mult, the F function in the fiestel network
/// x: input block
/// y: key
pub fn f(x: u128, y: u128) -> u128 {
    // w: the value we mult by 2 each time
    let mut w = x;
    // z: the output
    let mut z = 0;

    for i in 0..128 {
        // c: Carry
        let c = w >> 127;
        w <<= 1;

        // If there is a carry, add the modulo
        w ^= 0u128.wrapping_sub(c) & P;

        // Mult if required
        z ^= 0u128.wrapping_sub((y >> i) & 1) & w;
    }

    z
}

// Get all round keys from a vec
pub fn rk(x: &[u8]) -> [u128; 19] {
    let mut x2 = [0u128; 4];

    x.chunks_exact(16)
        .map(|x| u128::from_le_bytes(x.try_into().unwrap()))
        .enumerate()
        .for_each(|(i, x)| x2[i] = x);

    d(x2)
}

/// Key derivation
/// Get 12 round keys from the key
/// It's a bad key derivation, but i don't care
fn d(x: [u128; 4]) -> [u128; 19] {
    let mut x = x.clone();
    let mut z: [u128; 19] = Default::default();
    for i in 0..19 {
        for y in 0..4 {
            x[y] ^= x[y] << 92;
            x[y] ^= x[y] >> 65;

            z[i] ^= x[y];
        }
    }

    z
}
