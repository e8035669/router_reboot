use aes;
use aes::cipher::inout::block_padding::generic_array::typenum;
use aes::cipher::inout::block_padding::Pkcs7;
use aes::cipher::inout::InOutBufReserved;
use aes::cipher::{KeyIvInit, StreamCipher};
use ctr;
use hex;
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use sha2::Sha256;
use std::error::Error;

type Aes256Ctr64BE = ctr::Ctr64BE<aes::Aes256>;

fn aes_ctr_hash(message: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = message.to_vec();
    let msg_len = buffer.len();
    let new_len = (msg_len / 16 + 1) * 16;
    buffer.resize(new_len, 0);

    let inout = InOutBufReserved::from_mut_slice(buffer.as_mut_slice(), msg_len)?;
    let mut blocks = inout.into_padded_blocks::<Pkcs7, typenum::U16>()?;

    let mut cipher = Aes256Ctr64BE::new(key.into(), iv.into());

    for block in blocks.get_blocks() {
        cipher.apply_keystream_inout(block.into_buf());
    }

    if let Some(block) = blocks.get_tail_block() {
        cipher.apply_keystream_inout(block.into_buf());
    }
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let _r = "4BD2DB013D31B768A8D080AE44506F26";
    let _e = "0805503FAC292A7643B3F73FD3B8B523";
    let _privatekey = "A2F87350916AB32BDD6DC52779D9D259747FBDEF9DB236F0A7182EFC5BB536C2";
    let _return = "53B34DD5F65B1B2A32C41ABFCCE2B951B579527D84F7E6CD679767A061612BF017E3E3DB733BF42F1C70DF81154A6E48 0805503FAC292A7643B3F73FD3B8B523";
    let _encrypted = "53B34DD5F65B1B2A32C41ABFCCE2B951B579527D84F7E6CD679767A061612BF017E3E3DB733BF42F1C70DF81154A6E48";

    let key = hex::decode(_privatekey)?;
    let iv = hex::decode(_e)?;
    println!("{:?}", key);
    println!("{:?}", iv);

    let hashed = aes_ctr_hash(_r.as_bytes(), key.as_slice(), iv.as_slice())?;

    let ret = hex::encode_upper(hashed.as_slice());
    println!("answer: {:?}", _encrypted);
    println!("return: {:?}", ret);

    let hashed = Md5::new().chain_update("data").finalize().to_vec();
    println!("md5 hashed: {}", hex::encode_upper(hashed));

    let hashed = Hmac::<Sha256>::new_from_slice(_privatekey.as_bytes())?
        .chain_update("data")
        .finalize()
        .into_bytes()
        .to_vec();
    println!("hmac<sha256> hashed: {}", hex::encode_upper(hashed));

    Ok(())
}
