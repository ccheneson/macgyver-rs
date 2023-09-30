use crate::cli_args::NamespaceWithPods;
use crate::{get_k8s_env, Result};
use colored::{ColoredString, Colorize};
use k8s_openapi::api::core::v1::{ContainerStatus, Pod};
use kube::api::ListParams;
use kube::{api::Api, Client};
use std::ops::Deref;
use std::vec;

struct PodInfo<'a> {
    image: &'a str,
}

struct PrintArguments {
    image: String,
    status: String,
    restart_count: String,
    name: String,
}

struct ContainerStatusOps<'a> {
    container_status: &'a ContainerStatus,
}

impl<'a> ContainerStatusOps<'a> {
    fn new(container_status: &'a ContainerStatus) -> Self {
        Self { container_status }
    }

    fn restart_count(&self) -> ColoredString {
        let count = self.container_status.restart_count;
        if count != 0 {
            count.to_string().red()
        } else {
            count.to_string().white()
        }
    }

    fn status(&self) -> ColoredString {
        let status = self.container_status;
        let is_ready = status.ready;

        if let Some(container_state) = &status.state {
            match (
                &container_state.running,
                &container_state.terminated,
                &container_state.waiting,
            ) {
                //Running
                (Some(_), _, _) => "Running".to_string().green(),
                //Terminated
                (_, Some(terminated), _) => {
                    let reason = terminated
                        .reason
                        .as_ref()
                        .unwrap_or(&"N/A".to_string())
                        .yellow();
                    if !is_ready && reason.deref() == "Completed" {
                        "NotReady".to_string().yellow()
                    } else {
                        reason.trim().to_string().yellow()
                    }
                }
                //Waiting
                (_, _, Some(waiting)) => {
                    waiting.reason.as_ref().unwrap_or(&"N/A".to_string()).red()
                }
                //Default
                (_, _, _) => "N/A".white(),
            }
        } else {
            "N/A".white()
        }
    }
}

#[tokio::main]
pub async fn process(
    NamespaceWithPods {
        namespace,
        with_pod,
    }: NamespaceWithPods,
) -> Result<()> {
    let context = get_k8s_env()?;
    let client = Client::try_default().await?;
    let api: Api<Pod> = Api::namespaced(client, namespace.as_str());

    let headers = PrintArguments {
        image: "IMAGE".to_string(),
        status: "STATUS".white().to_string(),
        restart_count: "RESTART".white().to_string(),
        name: "NAME".to_string(),
    };

    let infos: Vec<Option<PrintArguments>> = api
        .list(&ListParams::default())
        .await?
        .into_iter()
        .map(|p| collect_info(p))
        .collect();

    let mut results: Vec<Option<PrintArguments>> = vec![Some(headers)];
    results.extend(infos);

    print_result(context.as_str(), &results, with_pod);

    Ok(())
}

fn print_result(context: &str, info: &[Option<PrintArguments>], with_pod: bool) {
    println!("ENVIRONMENT: {context}");
    info.into_iter().flatten().for_each(|i| {
        if with_pod {
            println!(
                "{0: <105} {1: <25} {2: <18} {3: <30}",
                i.image, i.status, i.restart_count, i.name
            )
        } else {
            println!(
                "{0: <105} {1: <25} {2: <18}",
                i.image, i.status, i.restart_count
            )
        }
    });
}

fn collect_info(p: Pod) -> Option<PrintArguments> {
    let spec = p.spec?;
    let name = p.metadata.name?;
    let status = p.status?;

    let containers = &spec.containers;

    let images: Vec<PodInfo> = containers
        .into_iter()
        .flat_map(|x| x.image.as_ref().map(|image| PodInfo { image }))
        .filter(|x| !x.image.contains("istio"))
        .collect();

    for image in images {
        if let Some(container_status) = &status.container_statuses {
            let maybe_image_status = container_status
                .into_iter()
                .filter(|status| image.image == status.image)
                .last();

            if let Some(image_status) = maybe_image_status {
                let status_ops = ContainerStatusOps::new(image_status);
                let arg = PrintArguments {
                    image: image.image.to_string(),
                    status: status_ops.status().to_string(),
                    restart_count: status_ops.restart_count().to_string(),
                    name: name.to_string(),
                };
                return Some(arg);
            }
        }
    }
    return None;
}
