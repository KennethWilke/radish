use std::net::SocketAddr;

#[allow(dead_code)]
pub enum ServerEncryption {
    Unencrypted,
    TlsEncrypted {
        cert: &'static str,
        key: &'static str,
    },
}

pub struct ServerConfiguration {
    pub addr: SocketAddr,
    pub encryption: ServerEncryption,
}

pub fn get_dev_configuration() -> ServerConfiguration {
    get_configuration(false, false)
}

pub fn get_prod_configuration() -> ServerConfiguration {
    get_configuration(true, true)
}

pub fn get_configuration(listen_all: bool, encrypted: bool) -> ServerConfiguration {
    let port = match encrypted {
        true => 8443,
        false => 8080,
    };

    let addr = match listen_all {
        true => SocketAddr::from(([0, 0, 0, 0], port)),
        false => SocketAddr::from(([127, 0, 0, 1], port)),
    };
    let encryption = match encrypted {
        true => ServerEncryption::TlsEncrypted {
            cert: "tls/fullchain.pem",
            key: "tls/key.pem",
        },
        false => ServerEncryption::Unencrypted,
    };
    ServerConfiguration { addr, encryption }
}

#[cfg(test)]
mod tests {
    use crate::config::{get_dev_configuration, get_prod_configuration};

    #[test]
    fn dev_config_expected() {
        let config = get_dev_configuration();
        assert_eq!(
            config.addr,
            crate::config::SocketAddr::from(([127, 0, 0, 1], 8080))
        );
    }

    #[test]
    fn prod_config_expected() {
        let config = get_prod_configuration();
        assert_eq!(
            config.addr,
            crate::config::SocketAddr::from(([0, 0, 0, 0], 8443))
        );
    }
}
