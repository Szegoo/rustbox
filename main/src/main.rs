use clap::Parser;

use cli::Cli;
use host::host;
use factory::Factory;
use traits::{FactoryT, Backer};

fn main() {
    let args = Cli::parse();

    let factory = Factory {};

    if args.download_uri.is_some() && args.download_name.is_some() {
        let backer = factory.make_backer();
        let uri = args.download_uri.expect("Error in the download uri.");
        let name = args.download_name.expect("Error in the file name.");
        backer.download_from(&uri, &name);
    }

    if args.host {
        host(args.port);
    }
}
