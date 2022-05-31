use clap::{Args, Error, ErrorKind, FromArgMatches, Parser};

mod deploy;
mod dev;
mod serve;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubcommand,
}

#[derive(Debug)]
enum CliSubcommand {
    Dev(DevArgs),
    Serve(ServeArgs),
    // Deploy,
}

#[derive(Parser, Debug)]
#[clap(about = "local development")]
pub struct DevArgs {
    #[clap(short, long, help = "http entry bind address")]
    bind: String,

    #[clap(short, long, parse(try_from_str = parse_modules_list), help = "List of modules to load")]
    module: Vec<(String, String)>,
}

#[derive(Parser, Debug)]
#[clap(about = "start production server")]
pub struct ServeArgs {
    #[clap(long, help = "http entry bind address")]
    http_entry_bind: String,

    #[clap(long, help = "external control bind address")]
    external_control_bind: String,

    #[clap(long, help = "db address, mariadb only")]
    db_url: String,
}

fn parse_modules_list(input: &str) -> Result<(String, String), &'static str> {
    let input = input.to_string();
    if let Some(idx) = input.find(":") {
        let (module, version) = input.split_at(idx);
        Ok((module.to_string(), version[1..].to_string()))
    } else {
        Err("expected format <module_name>:<module_file_path>")
    }
}

impl FromArgMatches for CliSubcommand {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        match matches.subcommand() {
            Some(("dev", args)) => Ok(Self::Dev(DevArgs::from_arg_matches(args)?)),
            Some(("serve", args)) => Ok(Self::Serve(ServeArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::UnrecognizedSubcommand,
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
            Some(("dev", args)) => *self = Self::Dev(DevArgs::from_arg_matches(args)?),
            Some(("serve", args)) => *self = Self::Serve(ServeArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Valid subcommands are `add` and `remove`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl clap::Subcommand for CliSubcommand {
    fn augment_subcommands(cmd: clap::Command<'_>) -> clap::Command<'_> {
        cmd.subcommand(DevArgs::augment_args(clap::Command::new("dev")))
            .subcommand(ServeArgs::augment_args(clap::Command::new("serve")))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: clap::Command<'_>) -> clap::Command<'_> {
        cmd.subcommand(DevArgs::augment_args(clap::Command::new("dev")))
            .subcommand(ServeArgs::augment_args(clap::Command::new("serve")))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "dev" | "serve")
    }
}

pub async fn boost() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    match args.subcommand {
        CliSubcommand::Dev(dev_args) => {
            dev::dev(dev_args).await?;
        }
        CliSubcommand::Serve(serve_args) => {
            serve::serve(serve_args).await?;
        }
    };
    Ok(())
}
