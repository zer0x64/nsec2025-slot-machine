/// Note: Names are non-descriptive on purpose to make it a bit harder to reverse
/// The cipher is a custom feistel network that uses a GF mul as the underlying PRF
mod common;

#[cfg(feature = "encrypt")]
mod encrypt;

#[cfg(feature = "decrypt")]
mod decrypt;

#[cfg(feature = "encrypt")]
pub use encrypt::encrypt;

#[cfg(feature = "decrypt")]
pub use decrypt::decrypt;

#[cfg(all(feature = "encrypt", feature = "decrypt", test))]
#[test]
fn test() {
    let input = b"this is a test input";
    let k = b"\xe4\x35\xf9\x95\x4d\x57\x67\xfd\xe4\x55\xe9\x2a\x94\xf7\x3c\x31\x1d\x32\xfa\x38\x14\x66\xa0\xac\xa9\x90\x2f\xfa\x8b\x4e\xbd\x49\x0c\x81\x39\xa6\x47\x8f\x43\xa7\x91\x51\xf6\x96\xae\x7a\x9f\x73\xf3\x16\x9d\xc6\xfb\x00\xa0\x83\xb5\xe2\x93\x19\xfb\xaf\x46\x93";

    let encrypted = encrypt(input, k);

    assert_ne!(&encrypted, &input);
    assert_ne!(&encrypted, &vec![0u8; encrypted.len()]);

    let decrypted = decrypt(&encrypted, k);

    assert_eq!(&decrypted[..input.len()], input);
}
