use clap::Parser;

/// Back up files from web or start hosting them.
#[derive(Parser)]
pub struct Cli {
    /// The URI of the file that the user wants to download.
    #[clap(short = 'u', long = "download-uri")]
    pub download_uri: Option<String>,

    /// The name of the newly downloaded file(without the extension specified). This is up to the
    /// user to decide. Note that this will be used for getting the file.
    #[clap(short = 'n', long = "download-name")]
    pub download_name: Option<String>,

    /// Flag that specifies whether to run the host.
    #[clap(long = "host")]
    pub host: bool,

    /// The port for the host, The default is 7878.
    #[clap(long = "port", default_value = "7878")]
    pub port: u16,
}
