use crate::cli_args::NamespaceArgs;
use crate::{get_k8s_env, Result};
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::ListParams;
use kube::{api::Api, Client};

#[tokio::main]
pub async fn process(NamespaceArgs { namespace }: NamespaceArgs) -> Result<()> {
    let context = get_k8s_env()?;
    let client = Client::try_default().await?;
    let api: Api<ConfigMap> = Api::namespaced(client, namespace.as_str());

    println!("ENVIRONMENT: {context}");

    api.list(&ListParams::default())
        .await?
        .into_iter()
        .for_each(|p| {
            if let Some(name) = p.metadata.name {
                println!("{name}: "); // print project
                if let Some(config) = p.data {
                    config.into_iter().for_each(|(key, value)| {
                        println!("      {0:_<50} {1: <10}", key, value);
                    })
                } else {
                    println!("      None");
                }
                println!()
            }
        });

    Ok(())
}
