use clap::Parser;

use cli::Cli;
use host::host;

fn main() {
    let args = Cli::parse();

    if args.download_uri.is_some() && args.download_name.is_some() {
        println!("Downloading...");
    }

    if args.host {
        host(args.port);
    }
}
