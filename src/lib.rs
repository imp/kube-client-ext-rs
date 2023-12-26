use k8s_openapi_ext as k8s;

use k8s::apiextensionsv1;
use k8s::apiregistrationv1;
use k8s::appsv1;
use k8s::batchv1;
use k8s::corev1;
use k8s::rbacv1;
use k8s::storagev1;
// use k8s::metav1;
use kube_client as client;

use client::api;

pub use ext::KubeClientExt;
pub use ext2::KubeClientExt2;

mod ext;
mod ext2;
