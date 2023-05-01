use clap::Parser;
use macgyver_rs::cli_args::Services;
use macgyver_rs::cli_args::{CliArgs, NamespaceWithPod, NamespaceArgs, NamespaceWithEncodedSecretArgs};
use macgyver_rs::{pods, cpumem, configmap, secret, logs};
use macgyver_rs::cli_args::Entities::Pods;
use macgyver_rs::cli_args::Entities::Configmap;
use macgyver_rs::cli_args::Entities::CpuMem;
use macgyver_rs::cli_args::Entities::Secret;
use macgyver_rs::cli_args::Entities::Logs;
use macgyver_rs::Result;


fn main() -> Result<()> {

    let args = CliArgs::parse();


    match args.entity {
        Pods(NamespaceWithPod { namespace, with_pod })=>
            pods::process(namespace.as_str(), with_pod )? ,
        Configmap(NamespaceArgs { namespace})=> 
            configmap::process(namespace.as_str())?,
        CpuMem(NamespaceArgs { namespace})=> 
            cpumem::process(namespace.as_str())?,
        Secret(NamespaceWithEncodedSecretArgs { namespace, with_encoded})=> 
            secret::process(namespace.as_str(), with_encoded)?,        
        Logs(Services { services , pattern})=>
            logs::process(&services, pattern)?
    };
    Ok(())
}