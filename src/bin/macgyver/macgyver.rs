use clap::Parser;
use macgyver_rs::cli_args::CliArgs;
use macgyver_rs::Result;

use macgyver_rs::cli_args::Entities::Pods;
use macgyver_rs::pods;

#[cfg(feature = "cpumem")]
use macgyver_rs::cli_args::Entities::CpuMem;
#[cfg(feature = "cpumem")]
use macgyver_rs::cpumem;

#[cfg(feature = "configmap")]
use macgyver_rs::cli_args::Entities::Configmap;
#[cfg(feature = "configmap")]
use macgyver_rs::configmap;

#[cfg(feature = "secret")]
use macgyver_rs::cli_args::Entities::Secret;
#[cfg(feature = "secret")]
use macgyver_rs::secret;

#[cfg(feature = "logs")]
use macgyver_rs::cli_args::Entities::Logs;
#[cfg(feature = "logs")]
use macgyver_rs::logs;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.entity {
        Pods(args) => pods::process(args)?,

        #[cfg(feature = "configmap")]
        Configmap(args) => configmap::process(args)?,

        #[cfg(feature = "cpumem")]
        CpuMem(args) => cpumem::process(args)?,

        #[cfg(feature = "secret")]
        Secret(args) => secret::process(args)?,

        #[cfg(feature = "logs")]
        Logs(args) => logs::process(args)?,
    };
    Ok(())
}
