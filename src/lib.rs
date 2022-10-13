use k8s_openapi_ext::apiextensionsv1;
use k8s_openapi_ext::apiregistrationv1;
use k8s_openapi_ext::appsv1;
use k8s_openapi_ext::corev1;
// use k8s_openapi_ext::metav1;
use kube_client as client;

use client::api;

pub use ext::KubeClientExt;
pub use ext2::KubeClientExt2;

mod ext;
mod ext2;
