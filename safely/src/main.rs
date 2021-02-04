use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::WriteColor;

mod config;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

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
    let opt = Opt::from_args();

    let _ = match opt.config {
        None => config::Config::load(),
        Some(path) => config::Config::from_file(path),
    }
    .unwrap_or_else(|error| {
        write_error(&format!("failed to load safely config: {:?}", error));
        std::process::exit(1);
    });

    match opt.command {
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
