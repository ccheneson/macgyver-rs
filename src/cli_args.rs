use clap::{Parser, Subcommand};
use std::io;

/// A CLI tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub entity: Entities,
}

#[derive(Subcommand, Debug)]
pub enum Entities {
    /// Collect info on pods
    Pods(NamespaceWithPods),

    /// Collect configmaps
    #[cfg(feature = "configmap")]
    Configmap(NamespaceArgs),

    /// Collect resources(cpu, requests) info
    #[cfg(feature = "cpumem")]
    CpuMem(NamespaceArgs),

    /// Collect secret info
    #[cfg(feature = "secret")]
    Secret(NamespaceWithEncodedSecretArgs),

    /// Collect logs through different services
    #[cfg(feature = "logs")]
    Logs(Services),
}

#[derive(Parser, Debug)]
#[clap(arg_required_else_help = true)]
pub struct Services {
    #[arg(short, long, value_parser = validate_services)]
    pub services: Vec<NamespaceAndService>,

    #[arg(short, long)]
    pub pattern: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct NamespaceAndService {
    pub namespace: String,
    pub service: String,
}

fn validate_services(args: &str) -> io::Result<NamespaceAndService> {
    let args_token: Vec<&str> = args.split(":").into_iter().collect();
    match (args_token.get(0), args_token.get(1)) {
        (Some(namespace), Some(service)) if args_token.len() == 2 => Ok(NamespaceAndService {
            namespace: namespace.to_string(),
            service: service.to_string(),
        }),
        (_, _) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid argument . Service has syntax namespace:service",
        )),
    }
}

#[derive(Parser, Debug)]
pub struct NamespaceWithPods {
    #[arg(long)]
    pub with_pod: bool,

    #[arg(short, long)]
    pub namespace: String,
}

#[derive(Parser, Debug)]
pub struct NamespaceArgs {
    #[arg(short, long)]
    pub namespace: String,
}

#[derive(Parser, Debug)]
pub struct NamespaceWithEncodedSecretArgs {
    #[arg(long)]
    pub with_encoded: bool,

    #[arg(short, long)]
    pub namespace: String,
}
