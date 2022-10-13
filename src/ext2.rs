use super::*;

/// Async extentions to `kube::Client`
///
#[async_trait::async_trait(?Send)]
pub trait KubeClientExt2: KubeClientExt {
    /// Get named deployment from a given (or default) namespace
    ///
    async fn get_deployment(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Option<appsv1::Deployment>> {
        self.deployments(namespace).get_opt(name).await
    }

    /// Get named api service
    ///
    async fn get_apiservice(
        &self,
        name: &str,
    ) -> client::Result<Option<apiregistrationv1::APIService>> {
        self.apiservices().get_opt(name).await
    }
}

impl KubeClientExt2 for client::Client {}
