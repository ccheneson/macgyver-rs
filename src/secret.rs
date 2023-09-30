use crate::cli_args::NamespaceWithEncodedSecretArgs;
use crate::{check_secret, get_k8s_env, Result};
use base64::{engine, Engine as _};
use colored::Colorize;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client};
use std::{fmt::Display, str};

#[tokio::main]
pub async fn process(
    NamespaceWithEncodedSecretArgs {
        namespace,
        with_encoded,
    }: NamespaceWithEncodedSecretArgs,
) -> Result<()> {
    check_secret()?;
    let context = get_k8s_env()?;
    let client = Client::try_default().await?;
    let api: Api<Secret> = Api::namespaced(client, namespace.as_str());

    println!("ENVIRONMENT: {context}");

    api.list(&ListParams::default())
        .await?
        .into_iter()
        .filter(|x| x.type_.as_ref().unwrap() == "Opaque")
        .for_each(|s| {
            print_secret(s, with_encoded);
        });
    Ok(())
}

fn print_secret(s: Secret, with_encoded: bool) -> Option<()> {
    if let (Some(name), Some(data)) = (s.metadata.name, s.data) {
        println!("{}", name.bold().white());
        data.iter().for_each(|(key, value)| {
            let value_byte = value.0.as_ref();
            let base64 = engine::general_purpose::STANDARD.encode(&value_byte);
            let decoded = std::str::from_utf8(&value_byte).unwrap_or("Error decoding");
            let secret = Decoded {
                key,
                decoded,
                base64: base64.as_str(),
                with_encoded,
            };
            println!("{}", secret);
        })
    }
    println!();

    None
}

struct Decoded<'a> {
    key: &'a str,
    decoded: &'a str,
    base64: &'a str,
    with_encoded: bool,
}

impl<'a> Display for Decoded<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.with_encoded {
            write!(
                f,
                "      {0:_<50} {1:_<70} {2}",
                self.key, self.decoded, self.base64
            )
        } else {
            write!(f, "      {0:_<50} {1}", self.key, self.decoded)
        }
    }
}
