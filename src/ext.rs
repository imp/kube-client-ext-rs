use k8s_openapi_ext::apiextensionsv1;
use k8s_openapi_ext::appsv1;
use k8s_openapi_ext::corev1;
// use k8s_openapi_ext::metav1;
use kube::api;

pub trait KubeClientExt: Clone {
    fn delete_params(&self) -> api::DeleteParams {
        api::DeleteParams {
            grace_period_seconds: Some(0),
            ..api::DeleteParams::default()
        }
    }

    fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
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
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default;

    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default;

    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default;

    fn nodes(&self) -> api::Api<corev1::Node> {
        self.api::<corev1::Node>()
    }

    fn namespaces(&self) -> api::Api<corev1::Namespace> {
        self.api::<corev1::Namespace>()
    }

    fn crds(&self) -> api::Api<apiextensionsv1::CustomResourceDefinition> {
        self.api::<apiextensionsv1::CustomResourceDefinition>()
    }

    fn configmaps<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::ConfigMap> {
        self.k(namespace)
    }

    fn daemonsets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<appsv1::DaemonSet> {
        self.k(namespace)
    }

    fn deployments<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::Deployment> {
        self.k(namespace)
    }

    fn secrets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Secret> {
        self.k(namespace)
    }

    fn pods<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Pod> {
        self.k(namespace)
    }

    fn k<'a, K>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        if let Some(namespace) = namespace.into() {
            self.namespaced_api::<K>(namespace)
        } else {
            self.default_namespaced_api::<K>()
        }
    }
}

impl KubeClientExt for kube::Client {
    fn api<K>(&self) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        api::Api::<K>::all(self.clone())
    }

    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        api::Api::<K>::default_namespaced(self.clone())
    }

    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        api::Api::<K>::namespaced(self.clone(), namespace)
    }
}
