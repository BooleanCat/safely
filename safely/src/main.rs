use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(no_version)]
    Create {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "path-to-bundle", parse(from_os_str))]
        bundle_path: PathBuf,
    },

    #[structopt(no_version)]
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(no_version)]
    Start {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(no_version)]
    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "signal", default_value = "SIGTERM")]
        signal: String,
    },

    #[structopt(no_version)]
    Delete {
        #[structopt(name = "container-id")]
        id: String,
    },
}

fn main() {
    match Opt::from_args().command {
        Subcommand::Create {
            id: _,
            bundle_path: _,
        } => unimplemented!(),
        Subcommand::State { id: _ } => unimplemented!(),
        Subcommand::Start { id: _ } => unimplemented!(),
        Subcommand::Kill { id: _, signal: _ } => unimplemented!(),
        Subcommand::Delete { id: _ } => unimplemented!(),
    }
}
