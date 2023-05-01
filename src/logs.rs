use std::io::Read;
use bytes::Bytes;
use colored::Colorize;
use k8s_openapi::api::core::v1::{Pod, Container};
use kube::api::{ListParams, LogParams};
use kube::{Client, Api};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use tokio::sync::mpsc::{self, Sender}; // for `next`
use crate::cli_args::NamespaceAndService;
use crate::{Result, get_k8s_env};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct K8sLogOutputRaw {
    service: String,
    raw: Vec<u8>
}

async fn get_pods_per_service(api: &Api<Pod>, service_name: String) -> Result<Vec<Pod>> {
    let label = format!("app.kubernetes.io/instance={}", service_name);
    let pods  = api.list(&ListParams::default().
            labels(label.as_str())
    )
    .await?
    .into_iter()
    .collect::<Vec<Pod>>();
    Ok(pods)
}


fn build_log_params(pod: Pod, service: &str) -> Option<(String, LogParams)> {
    let params_default = LogParams::default();
    let name = pod.metadata.name?;
        pod.spec?
        .containers
        .iter()
        .filter(|container| container.name.contains(service))
        .collect::<Vec<&Container>>()
        .first()
        .map(|container|  
            (name, LogParams { 
                container: Some(container.name.to_string()), follow: true, ..params_default
            })
        )
}


async fn send_message_to_channel(service: String, tx: &Sender<K8sLogOutputRaw>, message: Bytes) {
    let data: std::result::Result<Vec<_>, _>  = message.bytes().collect();
    if let std::result::Result::Ok(bytes) = data {
        let log = K8sLogOutputRaw { service, raw :  bytes };
        tx.send(log).await.ok();
    }
}

#[tokio::main]
pub  async fn process(services: &[NamespaceAndService], pattern: Option<String>) -> Result<()> {
    let context = get_k8s_env()?;
    let client = Client::try_default().await?;
    let (tx, mut rx) = mpsc::channel(200);


    for NamespaceAndService { namespace, service } in services.into_iter() {
        let api: Api<Pod> = Api::namespaced(client.clone(), namespace);
        let service_name = format!("{}-{}", namespace, service);
        let pods = 
            get_pods_per_service(&api, service_name)
                .await?
                .into_iter()
                .flat_map(|pod| build_log_params(pod, service.as_str())
        );

        pods
            .into_iter()
            .for_each(|(service, log_params)| {
                let local_api: Api<Pod> = api.clone();
                let local_tx = tx.clone();
                tokio::spawn(async move {
                    let mut log_stream= 
                        local_api.log_stream(service.as_str(), &log_params)
                                 .await
                                 .expect("Trying to read log stream");
                    while let Some(Ok(data)) = log_stream.next().await {
                        send_message_to_channel(service.to_string(), &local_tx, data).await;
                    }
                });
            });      
    }
    
    while let Some(log) = rx.recv().await {
        let result_log_string = String::from_utf8(log.raw);
        if let Ok(mut log_string) = result_log_string {
            log_string.pop(); //Remove new line char
            match pattern {
                Some(ref p) if log_string.contains(p) => println!("{}-{} - {}", context.green(), log.service.yellow(), log_string),
                Some(_) => (),
                None => println!("{}-{} - {}", context.green(), log.service.yellow(), log_string)
            }
        }            
    }  

    Ok(())
            
}

