mod utils;

use dotenv_codegen::dotenv;

use std::error::Error;

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    let username = dotenv!("USERNAME");
    let password = dotenv!("PASSWORD");

    let mut router = Router::new("http://192.168.1.1/cgi-bin/wwwctrl.cgi")?;
    router.login(username, password)?;
    router.reboot()?;
    Ok(())
}
