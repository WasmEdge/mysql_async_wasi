#![cfg(any(
    feature = "native-tls",
    feature = "rustls-tls",
    feature = "wasmedge-tls"
))]

pub mod native_tls_error;
pub mod rustls_error;

#[cfg(feature = "native-tls")]
pub use native_tls_error::TlsError;

#[cfg(feature = "rustls")]
pub use rustls_error::TlsError;

#[cfg(feature = "wasmedge-tls")]
pub use wasmedge_rustls_api::TlsError;
