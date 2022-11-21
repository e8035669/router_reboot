mod utils;

use clap::{Parser, Subcommand};
use dotenv_codegen::dotenv;
use std::error::Error;

use utils::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Reboot,
    Sms,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let dlink_loginpw = dotenv!("DLINK_PASSWORD");
    let mut dlink = DLinkRouter::new("http://192.168.1.254/DHMAPI/")?;
    dlink.login(dlink_loginpw)?;
    println!("Login success");

    match &cli.commands {
        Commands::Reboot => {
            dlink.reboot()?;
            println!("Send reboot success");
        }
        Commands::Sms => {
            let sms = dlink.get_sms()?;
            println!("SMS:\n{}", sms);
        }
    }

    Ok(())
}
