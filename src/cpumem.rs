use crate::cli_args::NamespaceArgs;
use crate::{get_k8s_env, Result};
use k8s_openapi::api::core::v1::{Pod, ResourceRequirements};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use kube::api::ListParams;
use kube::{api::Api, Client};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::vec;

#[derive(Debug)]
struct PrintArguments {
    pod: String,
    limit_cpu: String,
    limit_memory: String,
    request_cpu: String,
    request_memory: String,
}

impl Display for PrintArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0: <100} {1: <20} {2: <20} {3: <20} {4: <20}",
            self.pod, self.limit_cpu, self.limit_memory, self.request_cpu, self.request_memory
        )
    }
}

#[tokio::main]
pub async fn process(NamespaceArgs { namespace }: NamespaceArgs) -> Result<()> {
    let context = get_k8s_env()?;
    let client = Client::try_default().await?;
    let api: Api<Pod> = Api::namespaced(client, namespace.as_str());

    let headers = PrintArguments {
        pod: "pod".to_string(),
        limit_cpu: "limit.cpu".to_string(),
        limit_memory: "limit.memory".to_string(),
        request_cpu: "request.cpu".to_string(),
        request_memory: "request.memory".to_string(),
    };

    let infos: Vec<Option<PrintArguments>> = api
        .list(&ListParams::default())
        .await?
        .into_iter()
        .map(|p| collect_info(p))
        .collect();

    let mut results: Vec<Option<PrintArguments>> = vec![Some(headers)];
    results.extend(infos);

    print_result(context.as_str(), &results);

    Ok(())
}

fn print_result(context: &str, info: &[Option<PrintArguments>]) {
    println!("ENVIRONMENT: {context}");
    info.into_iter()
        .flatten()
        .for_each(|item| println!("{}", item))
}

fn collect_info(p: Pod) -> Option<PrintArguments> {
    let spec = p.spec?;
    let name = p.metadata.name?;

    let containers: Vec<&ResourceRequirements> = spec
        .containers
        .iter()
        .filter(|x| name.contains(&x.name))
        .flat_map(|c| c.resources.as_ref())
        .collect();

    if let Some(resource) = containers.first() {
        let limits = resource.limits.as_ref();
        let requests = resource.requests.as_ref();

        let cpu_mem = PrintArguments {
            pod: name,
            limit_cpu: extract_info(limits, "cpu"),
            limit_memory: extract_info(limits, "memory"),
            request_cpu: extract_info(requests, "cpu"),
            request_memory: extract_info(requests, "memory"),
        };
        Some(cpu_mem)
    } else {
        None
    }
}

fn extract_info(btree: Option<&BTreeMap<String, Quantity>>, key: &str) -> String {
    match btree {
        Some(info) => info
            .get(key)
            .map(|x| x.0.to_string())
            .unwrap_or("N/A".to_string()),
        None => "N/A".to_string(),
    }
}
