use std::{error, fmt, string::FromUtf8Error};
use gitlab::{GitlabError, api::{projects::repository::TreeBuilderError, ApiError}};
use kube::config::{KubeconfigError, InferConfigError};


#[derive(Debug)]
pub enum MacgyverCmdErrors {
    CliParameterMissing(String),
    KubeConfig(KubeconfigError),
    K8sCanNotCreateClient(InferConfigError),
    K8sBuildRequest(kube::Error),
    GitlabCanNotCreateClient(GitlabError),
    GitlabScope(TreeBuilderError),
    GitlabBuildRequest(kube::Error),
    GitlabPaged(String),
    HttpClient(reqwest::Error),
    Utf8Conversion(FromUtf8Error),
    YamlError(serde_yaml::Error),
    Processing(Box<dyn error::Error>)
}

impl fmt::Display for MacgyverCmdErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacgyverCmdErrors::CliParameterMissing(ref error) => write!(f, "Missing parameters/env vars: {}", error.to_string()),
            MacgyverCmdErrors::KubeConfig(ref error) => write!(f, "Error KubeConfig: {}", error.to_string()),
            MacgyverCmdErrors::K8sCanNotCreateClient(ref error) => write!(f, "Can not create k8s client: {}", error.to_string()),
            MacgyverCmdErrors::K8sBuildRequest(ref error) => write!(f, "Error when building k8s request: {}", error.to_string()),
            MacgyverCmdErrors::GitlabScope(ref error) => write!(f, "Error when GitlabScope API response: {}", error.to_string()),
            MacgyverCmdErrors::GitlabCanNotCreateClient(ref error) => write!(f, "Error when creating gitlab client: {}", error.to_string()),
            MacgyverCmdErrors::GitlabBuildRequest(ref error) => write!(f, "Error when building gitlab request: {}", error.to_string()),
            MacgyverCmdErrors::GitlabPaged(ref error) => write!(f, "Error when using the page api from gitlab: {}", error.to_string()),
            MacgyverCmdErrors::YamlError(ref error) => write!(f, "Error when deserializing yaml: {}", error.to_string()),
            MacgyverCmdErrors::Processing(ref error) => write!(f, "Error while processing: {}", error.to_string()),
            MacgyverCmdErrors::HttpClient(ref error) => write!(f, "Error while doing as client http request: {}", error.to_string()),
            MacgyverCmdErrors::Utf8Conversion(ref error) => write!(f, "Error while converting utf8 to string: {}", error.to_string()),
        }
    }
}

impl error::Error for MacgyverCmdErrors { 

}

impl From<InferConfigError> for MacgyverCmdErrors  {
    fn from(error: InferConfigError) -> Self {
        MacgyverCmdErrors::K8sCanNotCreateClient(error)
    }
}

impl From<kube::Error> for MacgyverCmdErrors  {
    fn from(error: kube::Error) -> Self {
        MacgyverCmdErrors::K8sBuildRequest(error)
    }
}

impl  From<Box<dyn error::Error>> for MacgyverCmdErrors  {
    fn from(error: Box<dyn error::Error>) -> Self {
        MacgyverCmdErrors::Processing(error)
    }
}

impl From<KubeconfigError> for MacgyverCmdErrors  {
    fn from(error: KubeconfigError) -> Self {
        MacgyverCmdErrors::KubeConfig(error)
    }
}

impl  From<TreeBuilderError> for MacgyverCmdErrors  {
    fn from(error: TreeBuilderError) -> Self {
        MacgyverCmdErrors::GitlabScope(error)
    }
}

impl<E: error::Error + Send + Sync + 'static>  From<ApiError<E>> for MacgyverCmdErrors  {
    fn from(error: ApiError<E>) -> Self {
        MacgyverCmdErrors::GitlabPaged(error.to_string())
    }
}

impl From<reqwest::Error> for MacgyverCmdErrors  {
    fn from(error: reqwest::Error) -> Self {
        MacgyverCmdErrors::HttpClient(error)
    }
}


impl From<serde_yaml::Error> for MacgyverCmdErrors  {
    fn from(error: serde_yaml::Error) -> Self {
        MacgyverCmdErrors::YamlError(error)
    }
}

impl From<FromUtf8Error> for MacgyverCmdErrors  {
    fn from(error: FromUtf8Error) -> Self {
        MacgyverCmdErrors::Utf8Conversion(error)
    }
}

impl From<GitlabError> for MacgyverCmdErrors  {
    fn from(error: GitlabError) -> Self {
        MacgyverCmdErrors::GitlabCanNotCreateClient(error)
    }
}

