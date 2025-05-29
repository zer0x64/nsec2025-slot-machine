# Slot Machine Crypto Module

The `slot-machine-crypto` module provides cryptographic functionality for the Wonderlight Slot Machine project, used for obfuscation of the second reversing flag.

## Technical details

The encryption algorithm is a custom Feistel network based on Galois Field multiplications of the state vector with the subkeys. It uses ECB mode with PKCS7 padding. It is likely very unsafe and should not be used for any real-world cryptographic purposes.

## License

This module is dual-licensed under either:

- MIT License ([LICENSE-MIT](../LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))

at your option.
