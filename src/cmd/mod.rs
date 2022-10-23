use clap::{error::ErrorKind, Args, Error, FromArgMatches, Parser};

mod run;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubcommand,
}

#[derive(Debug)]
enum CliSubcommand {
    Run(RunArgs),
}

#[derive(Parser, Debug)]
#[clap(about = "run")]
pub struct RunArgs {
    #[clap(short, long, help = "run server", required = true)]
    config: String,

    #[clap(short, long, help = "development flag", default_value = "false")]
    dev: bool,
}

impl FromArgMatches for CliSubcommand {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        match matches.subcommand() {
            Some(("run", args)) => Ok(Self::Run(RunArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        match matches.subcommand() {
            Some(("run", args)) => *self = Self::Run(RunArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `dev` and `serve`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl clap::Subcommand for CliSubcommand {
    fn augment_subcommands(cmd: clap::Command) -> clap::Command {
        cmd.subcommand(RunArgs::augment_args(clap::Command::new("run")))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: clap::Command) -> clap::Command {
        cmd.subcommand(RunArgs::augment_args(clap::Command::new("run")))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "dev" | "serve")
    }
}

pub async fn boost() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    match args.subcommand {
        CliSubcommand::Run(run_args) => {
            run::run(run_args).await?;
        }
    };
    Ok(())
}
