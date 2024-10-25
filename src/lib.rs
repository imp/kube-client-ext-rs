#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
// #![warn(rust_2024_compatibility)]
#![warn(rust_2024_incompatible_pat)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use k8s_openapi_ext as k8s;

use k8s::apiextensionsv1;
use k8s::apiregistrationv1;
use k8s::appsv1;
use k8s::autoscalingv2;
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
