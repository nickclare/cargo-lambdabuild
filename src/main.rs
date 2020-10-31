use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    // top level arguments
    #[structopt(short)]
    verbose: bool,

    #[structopt(subcommand)]
    lambdabuild: LambdaBuilderOpts,
}

// actual commands
#[derive(StructOpt, Debug)]
enum LambdaBuilderOpts {
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
    println!("{:?}", opt);
    Ok(())
}
