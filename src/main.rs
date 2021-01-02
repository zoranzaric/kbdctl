extern crate hidapi;

use hidapi::HidApi;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kbdctl", about = "a tool to control my keyboard")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Led {
        #[structopt(subcommand)]
        command: LedCommand,
    },
}
#[derive(StructOpt)]
#[structopt(about = "on or off")]
enum LedCommand {
    #[structopt(about = "turn led on")]
    On,
    #[structopt(about = "turn led off")]
    Off,
}

fn main() {
    let opt = Opt::from_args();

    let x = match opt.command {
        Command::Led {
            command: led_command,
        } => match led_command {
            LedCommand::On => 1,
            LedCommand::Off => 0,
        },
    };
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == 0xfeed
                    && device.product_id() == 0
                    && device.usage_page() == 0xff60
                {
                    match device.open_device(&api) {
                        Ok(d) => {
                            let _ = d.write(&[x, 0]);
                            std::process::exit(0);
                        }
                        Err(e) => {
                            println!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
