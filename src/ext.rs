use super::*;

pub trait KubeClientExt: Clone {
    fn delete_params(&self) -> api::DeleteParams {
        api::DeleteParams::default().grace_period(0)
    }

    fn background_delete(&self) -> api::DeleteParams {
        api::DeleteParams::background().grace_period(0)
    }

    fn foreground_delete(&self) -> api::DeleteParams {
        api::DeleteParams::foreground().grace_period(0)
    }

    fn orphan_delete(&self) -> api::DeleteParams {
        api::DeleteParams::orphan().grace_period(0)
    }

    fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
    }

    fn watch_params(&self) -> api::WatchParams {
        api::WatchParams::default()
    }

    fn post_params(&self) -> api::PostParams {
        api::PostParams::default()
    }

    fn post_params_with_manager(&self, manager: &str) -> api::PostParams {
        let mut pp = self.post_params();
        pp.field_manager = Some(manager.to_string());
        pp
    }

    fn patch_params(&self) -> api::PatchParams {
        api::PatchParams::default()
    }

    fn patch_params_with_manager(&self, manager: &str) -> api::PatchParams {
        api::PatchParams::apply(manager)
    }

    fn log_params(&self) -> api::LogParams {
        api::LogParams::default()
    }

    fn log_params_previous(&self) -> api::LogParams {
        api::LogParams {
            previous: true,
            ..api::LogParams::default()
        }
    }
    fn api<K>(&self) -> api::Api<K>
    where
        K: client::Resource,
        <K as client::Resource>::DynamicType: Default;

    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default;

    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default;

    fn apiservices(&self) -> api::Api<apiregistrationv1::APIService> {
        self.api()
    }

    fn clusterroles(&self) -> api::Api<rbacv1::ClusterRole> {
        self.api()
    }

    fn clusterrolebindings(&self) -> api::Api<rbacv1::ClusterRoleBinding> {
        self.api()
    }

    fn crds(&self) -> api::Api<apiextensionsv1::CustomResourceDefinition> {
        self.api()
    }

    fn nodes(&self) -> api::Api<corev1::Node> {
        self.api()
    }

    fn namespaces(&self) -> api::Api<corev1::Namespace> {
        self.api()
    }

    fn persistentvolumes(&self) -> api::Api<corev1::PersistentVolume> {
        self.api()
    }

    fn storageclasses(&self) -> api::Api<storagev1::StorageClass> {
        self.api()
    }

    fn configmaps<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::ConfigMap> {
        self.namespaced_k(namespace)
    }

    fn daemonsets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<appsv1::DaemonSet> {
        self.namespaced_k(namespace)
    }

    fn deployments<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::Deployment> {
        self.namespaced_k(namespace)
    }

    fn horizontalpodautoscalers<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<autoscalingv2::HorizontalPodAutoscaler> {
        self.namespaced_k(namespace)
    }

    fn jobs<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<batchv1::Job> {
        self.namespaced_k(namespace)
    }

    fn cronjobs<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<batchv1::CronJob> {
        self.namespaced_k(namespace)
    }

    fn persistentvolumeclaims<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<corev1::PersistentVolumeClaim> {
        self.namespaced_k(namespace)
    }

    fn replicasets<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::ReplicaSet> {
        self.namespaced_k(namespace)
    }

    fn roles<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<rbacv1::Role> {
        self.namespaced_k(namespace)
    }

    fn rolebindings<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<rbacv1::RoleBinding> {
        self.namespaced_k(namespace)
    }
    fn secrets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Secret> {
        self.namespaced_k(namespace)
    }

    fn pods<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Pod> {
        self.namespaced_k(namespace)
    }

    fn serviceaccounts<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<corev1::ServiceAccount> {
        self.namespaced_k(namespace)
    }

    fn services<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Service> {
        self.namespaced_k(namespace)
    }

    fn statefulsets<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::StatefulSet> {
        self.namespaced_k(namespace)
    }

    fn namespaced_k<'a, K>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        if let Some(namespace) = namespace.into() {
            self.namespaced_api::<K>(namespace)
        } else {
            self.default_namespaced_api::<K>()
        }
    }
}

impl KubeClientExt for client::Client {
    fn api<K>(&self) -> api::Api<K>
    where
        K: client::Resource,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::all(self.clone())
    }

    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::default_namespaced(self.clone())
    }

    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::namespaced(self.clone(), namespace)
    }
}
