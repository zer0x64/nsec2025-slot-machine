use proc_macro2::TokenStream;
use quote::quote_spanned;
use rand::Rng;

pub fn obfuscate_flag_impl(flag: String, key: Vec<u8>) -> TokenStream {
    if flag.is_empty() {
        panic!("The flag is empty!");
    }
    if key.len() != 64 {
        panic!("The key must be 64 bytes long");
    }

    let ciphertext = slot_machine_crypto::encrypt(flag.as_bytes(), &key);
    let splitted_keys = split_key(&key);

    let span = proc_macro2::Span::call_site();

    let ct = syn::LitByteStr::new(&ciphertext, span);
    let k: Vec<syn::LitByteStr> = splitted_keys
        .iter()
        .map(|k| syn::LitByteStr::new(k.as_ref(), span))
        .collect();

    quote_spanned! { span =>
        let ct = #ct;
        let k: Vec<[u8; 64]> = vec![
            #(#k.clone(),)*
        ];

        let augw = AJBCKQOISJS.lock().await.to_vec();

        let k = &k[1..].iter().chain(augw.iter()).fold(k[0].to_vec(), |mut acc, k| {
            for i in 0..64 {
                acc[i] ^= k[i]
            }

            acc
        });

        return std::str::from_utf8(&slot_machine_crypto::decrypt(ct, &k)).unwrap().to_string();
    }
    .into()
}

fn split_key(k: &[u8]) -> Vec<[u8; 64]> {
    let runtime_added_keys: [[u8; 64]; 161] = crate::consts::RUNTIME_KEYS;

    let mut rng = rand::rng();

    let mut keys: Vec<[u8; 64]> = (0..161).map(|_| rng.random()).collect();

    let last_key = keys.iter().chain(runtime_added_keys.iter()).fold(
        <[u8; 64]>::try_from(k).unwrap(),
        |mut acc, k| {
            for i in 0..64 {
                acc[i] ^= k[i]
            }

            acc
        },
    );

    keys.push(last_key);

    keys
}
