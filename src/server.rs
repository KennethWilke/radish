use crate::config::{ServerEncryption::*, ServerConfiguration, get_dev_configuration};
use axum_server::tls_rustls::RustlsConfig;

pub struct RadishServer {
    configuration: ServerConfiguration,
}

impl RadishServer {
    pub fn new() -> Self {
        RadishServer {
            configuration: get_dev_configuration(),
        }
    }

    pub async fn run(&self, app: axum::Router) {
        let addr = self.configuration.addr;
        match self.configuration.encryption {
            Unencrypted => {
                println!("Running WITHOUT ENCRYPTION\nListening on {}", addr);

                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
            TlsEncrypted { cert, key } => {
                let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();
                println!("Running with Encryption\nListening on {}", addr);
                axum_server::bind_rustls(addr, config)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
        }
    }
}
