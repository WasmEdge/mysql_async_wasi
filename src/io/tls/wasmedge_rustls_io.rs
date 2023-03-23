#![cfg(feature = "wasmedge-tls")]

use wasmedge_rustls_api::{stream::async_stream::TlsStream, ClientConfig};

use crate::{io::Endpoint, Result};

impl Endpoint {
    pub async fn make_secure(&mut self, domain: String, _ssl_opts: crate::SslOpts) -> Result<()> {
        #[cfg(unix)]
        if self.is_socket() {
            // won't secure socket connection
            return Ok(());
        }

        let config = ClientConfig::default();

        *self = match self {
            Endpoint::Plain(ref mut stream) => {
                let stream = stream.take().unwrap();

                let connection = TlsStream::connect(&config, domain, stream)
                    .await
                    .map_err(|e| e.0)?;

                Endpoint::Secure(connection)
            }
            Endpoint::Secure(_) => unreachable!(),
            #[cfg(unix)]
            Endpoint::Socket(_) => unreachable!(),
        };

        Ok(())
    }
}
