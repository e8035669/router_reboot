use aes;
use aes::cipher::{KeyIvInit, StreamCipher};
use ctr;
use hex;
use block_padding::{Pkcs7, Padding};
use std::error::Error;

type Aes256Ctr64LE = ctr::Ctr128LE<aes::Aes256>;

fn main() -> Result<(), Box<dyn Error>> {
    let _r = "4BD2DB013D31B768A8D080AE44506F26";
    let _e = "0805503FAC292A7643B3F73FD3B8B523";
    let _privatekey = "A2F87350916AB32BDD6DC52779D9D259747FBDEF9DB236F0A7182EFC5BB536C2";
    let _return = "53B34DD5F65B1B2A32C41ABFCCE2B951B579527D84F7E6CD679767A061612BF017E3E3DB733BF42F1C70DF81154A6E48 0805503FAC292A7643B3F73FD3B8B523";

    // let key = [0x42; 32];
    let key = hex::decode(_privatekey)?;
    // let iv = [0x24; 16];
    let iv = hex::decode(_e)?;
    let plaintext = *b"hello world! this is my plaintext.";
    println!("{:?}", key);
    println!("{:?}", iv);
    println!("{:?}", plaintext);

    // let mut buf = plaintext.to_vec();
    let mut buf = _r.as_bytes().to_vec();
    let orig_size = buf.len();
    let mut cipher = Aes256Ctr64LE::new(key.as_slice().into(), iv.as_slice().into());

    cipher.apply_keystream(&mut buf);
    println!("{:?}", buf);
    let ret = hex::encode_upper(buf.as_slice());
    println!("ret: {}", ret);
    println!("ans: {}", _return);

    Ok(())
}
