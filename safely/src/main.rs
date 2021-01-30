use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::WriteColor;

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
        Subcommand::State { id: _ } => {
            write_error("no such container");
            std::process::exit(1);
        }
        Subcommand::Start { id: _ } => {
            write_error("no such container");
            std::process::exit(1);
        }
        Subcommand::Kill { id: _, signal: _ } => {
            write_error("no such container");
            std::process::exit(1);
        }
        Subcommand::Delete { id: _ } => {
            write_error("no such container");
            std::process::exit(1);
        }
    }
}

fn write_error(message: &str) {
    let mut stderr = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    stderr
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
        .unwrap();
    write!(&mut stderr, "error: ").unwrap();

    stderr.set_color(&termcolor::ColorSpec::new()).unwrap();

    eprintln!("{}", message);
}
