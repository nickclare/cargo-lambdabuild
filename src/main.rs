use cargo_metadata::MetadataCommand;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use structopt::StructOpt;
use zip::write::ZipWriter;

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

fn run_package(_opt: &Opt, _c: &CmdOpts) -> Result<(), anyhow::Error> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    let package = metadata
        .packages
        .iter()
        .find(|p| {
            p.targets
                .iter()
                .any(|t| t.kind.first().map_or(false, |e| e == "bin"))
        })
        .ok_or(anyhow::format_err!(
            "At least one workspace member should be a `bin' crate"
        ))?;
    let target_dir = metadata.target_directory;
    let binary_path = target_dir.join("x86_64-unknown-linux-musl").join("release");
    for t in package.targets.iter() {
        if t.kind.first().unwrap() == "bin" {
            let name = t.name.clone();
            build_package(&name, &binary_path.join(&name), &target_dir)?;
        }
    }
    Ok(())
}

fn build_package(name: &str, binary: &PathBuf, target_dir: &PathBuf) -> Result<(), anyhow::Error> {
    let dest_path = target_dir.join(format!("{}.zip", name));
    println!("packaging {:?} into zip named: {:?}", binary, dest_path);
    // input
    let mut infile = std::fs::File::open(binary)?;
    let mut zip = ZipWriter::new(std::fs::File::create(dest_path)?);
    zip.start_file("bootstrap", zip::write::FileOptions::default())?;
    std::io::copy(&mut infile, &mut zip)?;
    zip.finish()?;

    Ok(())
}
