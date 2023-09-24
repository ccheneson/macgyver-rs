use clap::Parser;
use macgyver_rs::cli_args::{CliArgs, NamespaceWithPods, NamespaceArgs, NamespaceWithEncodedSecretArgs};
use macgyver_rs::Result;

use macgyver_rs::pods;
use macgyver_rs::cli_args::Entities::Pods;

#[cfg(feature = "cpumem")]
use macgyver_rs::cpumem;
#[cfg(feature = "cpumem")]
use macgyver_rs::cli_args::Entities::CpuMem;

#[cfg(feature = "configmap")]
use macgyver_rs::configmap;
#[cfg(feature = "configmap")]
use macgyver_rs::cli_args::Entities::Configmap;


#[cfg(feature = "secret")]
use macgyver_rs::secret;
#[cfg(feature = "secret")]
use macgyver_rs::cli_args::Entities::Secret;


#[cfg(feature = "logs")]
use macgyver_rs::logs;
#[cfg(feature = "logs")]
use macgyver_rs::cli_args::Entities::Logs;

fn main() -> Result<()> {

    let args = CliArgs::parse();


    match args.entity {
        Pods(NamespaceWithPods { namespace, with_pod })  => 
            pods::process(namespace.as_str(), with_pod )? ,
        #[cfg(feature = "configmap")]
        Configmap(NamespaceArgs { namespace})=> 
            configmap::process(namespace.as_str())?,
        #[cfg(feature = "cpumem")]
        CpuMem(NamespaceArgs { namespace})=> 
            cpumem::process(namespace.as_str())?,
        #[cfg(feature = "secret")]
        Secret(NamespaceWithEncodedSecretArgs { namespace, with_encoded})=> 
            secret::process(namespace.as_str(), with_encoded)?,        
        #[cfg(feature = "logs")]
        Logs(Services { services , pattern})=>
            logs::process(&services, pattern)?
    };
    Ok(())
}