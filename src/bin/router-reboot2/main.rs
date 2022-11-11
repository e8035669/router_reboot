mod utils;

use dotenv_codegen::dotenv;
use std::error::Error;

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    let dlink_loginpw = dotenv!("DLINK_PASSWORD");

    let mut dlink = DLinkRouter::new("http://192.168.1.254/DHMAPI/")?;
    dlink.login(dlink_loginpw)?;
    println!("Login success");

    let sms = dlink.get_sms()?;
    println!("SMS:\n{}", sms);

    Ok(())
}
