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
    NetworkCheck,
}

fn get_router() -> Result<DLinkRouter, Box<dyn Error>> {
    let dlink_loginpw = dotenv!("DLINK_PASSWORD");
    let mut dlink = DLinkRouter::new("http://192.168.1.254/DHMAPI/")?;
    dlink.login(dlink_loginpw)?;
    println!("Login success");
    Ok(dlink)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Reboot => {
            let mut dlink = get_router()?;
            dlink.reboot()?;
            println!("Send reboot success");
        }
        Commands::Sms => {
            let mut dlink = get_router()?;
            let sms = dlink.get_sms()?;
            println!("SMS:\n{}", sms);
        }
        Commands::NetworkCheck => {
            let mut checker = Checker::new();
            checker.set_on_failed(|| {
                let dlink = get_router();
                match dlink {
                    Ok(mut dlink) => {
                        let res = dlink.reboot();
                        match res {
                            Ok(()) => println!("Reboot router!"),
                            Err(err) => println!("Reboot fail!: {:?}", err),
                        }
                    }
                    Err(err) => {
                        println!("Router login failed: {:?}", err);
                    }
                }
            });
            checker.start_check();
        }
    }

    Ok(())
}
