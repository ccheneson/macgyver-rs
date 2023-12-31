use errors::MacgyverCmdErrors;
use kube::config::Kubeconfig;
use std::env;
pub mod cli_args;
pub mod errors;

pub mod pods;

#[cfg(feature = "configmap")]
pub mod configmap;

#[cfg(feature = "cpumem")]
pub mod cpumem;

#[cfg(feature = "secret")]
pub mod secret;

#[cfg(feature = "logs")]
pub mod logs;

pub const PASSCHECK: &str = "This-is-my-pass-phrase";

pub type Result<T> = core::result::Result<T, MacgyverCmdErrors>;

pub fn get_k8s_env() -> Result<String> {
    Kubeconfig::read()
        .map(|conf| {
            conf.current_context
                .map(|s| s.to_uppercase())
                .unwrap_or("N/A".to_string())
        })
        .map_err(|err| err.into())
}

pub fn check_secret() -> Result<()> {
    match env::var("MACGYVER_CMD_SECRET_CHECK") {
        Ok(val) if val == PASSCHECK => Ok(()),
        _ => {
            return Err(MacgyverCmdErrors::CliParameterMissing(
                "Missing/Invalid env var MACGYVER_CMD_SECRET_CHECK".to_string(),
            ))
        }
    }
}

pub fn check_gitlab_token() -> Result<String> {
    match env::var("MACGYVER_CMD_GITLAB_TOKEN") {
        Ok(token) => Ok(token),
        _ => {
            return Err(MacgyverCmdErrors::CliParameterMissing(
                "Missing env var MACGYVER_CMD_GITLAB_TOKEN".to_string(),
            ))
        }
    }
}
