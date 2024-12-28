use std::net::{IpAddr, SocketAddr};
use std::env;
use dotenv;

#[derive(Debug, Clone)]
pub struct GrpcServerConfig {
    pub address: IpAddr,
    pub port: u16,
    pub enable_reflection: bool,
}

impl GrpcServerConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let address = env::var("GRPC_SERVER_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1".to_string())
            .parse()?;

        let port = env::var("GRPC_SERVER_PORT")
            .unwrap_or_else(|_| "50051".to_string())
            .parse()?;

        let enable_reflection = env::var("GRPC_SERVER_ENABLE_REFLECTION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()?;

        Ok(Self {
            address,
            port,
            enable_reflection,
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }
}