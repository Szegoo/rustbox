use clap::Parser;

use cli::Cli;
use host::host;
use factory::Factory;
use traits::{FactoryT, Backer};

/// The main function of the rustbox project.
fn main() {
    let args = Cli::parse();

    let factory = Factory {};

    // The `backer` can only download the specified if all of the necessary data is provided from
    // the arguments.
    if args.download_uri.is_some() && args.download_name.is_some() {
        let backer = factory.make_backer();
        let uri = args.download_uri.expect("Error in the download uri.");
        let name = args.download_name.expect("Error in the file name.");
        backer.download_from(&uri, &name);
    }

    // Host if the `host` flag is specified, otherwise the program will just finish running.
    // Important NOTE: This needs to be at the end of the main function or run concurrently because
    // the function won't finish on its own because it will always await for new requests. 
    if args.host {
        host(args.port);
    }
}
