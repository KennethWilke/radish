use std::net::SocketAddr;

use crate::config::{ServerConfig, ServerEncryption::*};
use axum_server::tls_rustls::RustlsConfig;

pub struct RadishServer {
    configuration: ServerConfig,
}

impl RadishServer {
    pub fn new(configuration: ServerConfig) -> Self {
        RadishServer { configuration }
    }

    pub async fn run(&self, app: axum::Router) {
        let addr = self.configuration.addr;
        match self.configuration.encryption {
            Unencrypted => {
                println!("Running WITHOUT ENCRYPTION\nListening on {}", addr);

                axum::Server::bind(&addr)
                    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
