use std::fmt;

use super::*;

/// Async extentions to `kube::Client`
///
#[async_trait::async_trait(?Send)]
pub trait KubeClientExt2: KubeClientExt {
    /// Get named secret from a given (or default) namespace
    /// Return `None` if not found`
    ///
    async fn get_secret_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Option<corev1::Secret>> {
        self.secrets(namespace).get_opt(name).await
    }

    /// Get named secret from a given (or default) namespace
    ///
    async fn get_secret(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<corev1::Secret> {
        self.secrets(namespace).get(name).await
    }

    /// Get named deployment from a given (or default) namespace
    /// Return `None` if not found
    ///
    async fn get_deployment_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Option<appsv1::Deployment>> {
        self.deployments(namespace).get_opt(name).await
    }

    /// Get named deployment from a given (or default) namespace
    ///
    async fn get_deployment(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<appsv1::Deployment> {
        self.deployments(namespace).get(name).await
    }

    /// Get named api service
    /// Return `None` if not found
    ///
    async fn get_apiservice_opt(
        &self,
        name: &str,
    ) -> client::Result<Option<apiregistrationv1::APIService>> {
        self.apiservices().get_opt(name).await
    }

    /// Get named api service
    ///
    async fn get_apiservice(&self, name: &str) -> client::Result<apiregistrationv1::APIService> {
        self.apiservices().get(name).await
    }

    /// Get named CRD
    /// Return `None` if not found
    ///
    async fn get_crd_opt(
        &self,
        name: &str,
    ) -> client::Result<Option<apiextensionsv1::CustomResourceDefinition>> {
        self.crds().get_opt(name).await
    }

    /// Get named CRD
    ///
    async fn get_crd(
        &self,
        name: &str,
    ) -> client::Result<apiextensionsv1::CustomResourceDefinition> {
        self.crds().get(name).await
    }

    /// Get owner object from `ownerReference` assuming it is of kind `K`
    ///
    async fn get_owner_k<O, K>(&self, o: &O) -> client::Result<Option<K>>
    where
        O: client::ResourceExt,
        K: Clone
            + fmt::Debug
            + k8s::openapi::serde::de::DeserializeOwned
            + client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        let dynamic_default = K::DynamicType::default();
        let kind = K::kind(&dynamic_default);
        let namespace = o.namespace();
        if let Some(name) = o
            .owner_references()
            .iter()
            .find(|owner| owner.kind == kind)
            .map(|owner| &owner.name)
        {
            self.namespaced_k(namespace.as_deref()).get_opt(name).await
        } else {
            Ok(None)
        }
    }

    /// List all `Pod`s  in a given (or default) namespace
    ///
    async fn list_pods(
        &self,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Vec<corev1::Pod>> {
        self.list_k(namespace).await
    }

    /// List all `Deployment`s in a given (or default) namespace
    ///
    async fn list_deployments(
        &self,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Vec<appsv1::Deployment>> {
        self.list_k(namespace).await
    }

    /// List namespaced objects of kind `K` in a given (or default) namespace
    ///
    async fn list_k<K>(&self, namespace: impl Into<Option<&str>>) -> client::Result<Vec<K>>
    where
        K: Clone
            + fmt::Debug
            + k8s::openapi::serde::de::DeserializeOwned
            + client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        let lp = self.list_params();
        self.namespaced_k(namespace)
            .list(&lp)
            .await
            .map(|list| list.items)
    }
}

impl KubeClientExt2 for client::Client {}
