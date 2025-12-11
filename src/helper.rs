use client::core::Status;
use client::Error;

use super::*;

/// This helper is useful when you call .delete() and want to ignore "not found" errors
/// the NOT_FOUND status will be converted into an Ok variant with a Status object
///
/// Example:
/// ```no_run
/// # use kube_client_ext::ignore_not_found;
/// # use kube_client_ext::KubeClientExt;
/// # use kube_client::Client;
///
/// async fn delete_and_ignore_not_found(client: &Client) -> bool {
///     client
///         .namespaces()
///         .delete("nonexistent", &client.delete_params())
///         .await
///         .or_else(ignore_not_found)
///         .is_ok()
/// }
/// ```
pub fn ignore_not_found<K>(err: Error) -> client::Result<either::Either<K, Status>> {
    match err {
        Error::Api(err) if err.code == 404 => {
            let status = Status::failure(&err.message, &err.reason).with_code(err.code);
            Ok(either::Right(status))
        }
        other => Err(other),
    }
}
