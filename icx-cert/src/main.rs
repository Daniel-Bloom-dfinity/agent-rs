use anyhow::Result;
use clap::{crate_authors, crate_version, Clap};

mod pprint;

#[derive(Clap)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
)]
enum Command {
    /// Fetches the specified URL and pretty-prints the certificate.
    #[clap(name = "print")]
    PPrint {
        url: String,

        /// Specifies one or more encodings to accept.
        #[clap(long, multiple(true), number_of_values(1))]
        accept_encoding: Option<Vec<String>>,
    },
}

fn main() -> Result<()> {
    match Command::parse() {
        Command::PPrint {
            url,
            accept_encoding,
        } => pprint::pprint(url, accept_encoding),
    }
}
