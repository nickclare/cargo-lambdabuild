use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    // top level arguments
    #[structopt(short)]
    verbose: bool,

    #[structopt(subcommand)]
    lambdabuild: LambdaBuildOpts,
}

// actual commands
#[derive(StructOpt, Debug)]
enum LambdaBuildOpts {
    #[structopt(name = "lambdabuild")]
    LambdaBuild {
        #[structopt(subcommand)]
        cmd: CmdOpts,
    },
}

#[derive(StructOpt, Debug)]
enum CmdOpts {
    /// Build the project with the correct settings for Lambda
    Build {},
    /// Build Lambda zip packages for each binary produced
    Package {},
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    match &opt.lambdabuild {
        LambdaBuildOpts::LambdaBuild { cmd, .. } => match cmd {
            b @ CmdOpts::Build { .. } => run_build(&opt, b),
            p @ CmdOpts::Package { .. } => run_package(&opt, p),
        },
    }
}

fn run_build(_opt: &Opt, _c: &CmdOpts) -> Result<(), anyhow::Error> {
    Command::new(std::env::var("CARGO")?)
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-unknown-linux-musl")
        .stdin(Stdio::null())
        .status()?;
    Ok(())
}

fn run_package(_opt: &Opt, c: &CmdOpts) -> Result<(), anyhow::Error> {
    Ok(())
}
