use std::fmt;

use super::*;

/// Async extentions to `kube::Client`
///
#[async_trait::async_trait(?Send)]
pub trait KubeClientExt2: KubeClientExt {
    /// Get named secret from a given (or default) namespace
    ///
    async fn get_secret(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>>,
    ) -> client::Result<Option<corev1::Secret>> {
        self.secrets(namespace).get_opt(name).await
    }

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

    /// Get named CRD
    ///
    async fn get_crd(
        &self,
        name: &str,
    ) -> client::Result<Option<apiextensionsv1::CustomResourceDefinition>> {
        self.crds().get_opt(name).await
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
}

impl KubeClientExt2 for client::Client {}
