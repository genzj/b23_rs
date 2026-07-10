use hickory_resolver::TokioAsyncResolver;
use reqwest::dns::{Addrs, Name, Resolve};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct SecureResolver {
    domains: Vec<String>,
    resolver: Arc<TokioAsyncResolver>,
}

impl SecureResolver {
    pub fn new(domains: Vec<String>) -> Self {
        let resolver = TokioAsyncResolver::tokio_from_system_conf()
            .expect("Failed to create DNS resolver from system configuration");
        Self {
            domains,
            resolver: Arc::new(resolver),
        }
    }
}

impl Resolve for SecureResolver {
    fn resolve(&self, _name: Name) -> reqwest::dns::Resolving {
        let domains = self.domains.clone();
        let resolver = self.resolver.clone();

        Box::pin(async move {
            for domain in domains {
                if let Ok(lookup) = resolver.lookup_ip(domain.as_str()).await {
                    let mut addrs = Vec::new();
                    for ip in lookup.iter() {
                        addrs.push(SocketAddr::new(ip, 0));
                    }
                    if !addrs.is_empty() {
                        let addrs: Addrs = Box::new(addrs.into_iter());
                        return Ok(addrs);
                    }
                }
            }
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No upstream domains resolved successfully",
            )) as Box<dyn std::error::Error + Send + Sync>)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::dns::Name;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_secure_resolver_success() {
        let resolver = SecureResolver::new(vec!["example.com".to_string()]);
        let name = Name::from_str("some-random-domain.com").unwrap();

        let mut addrs = resolver.resolve(name).await.expect("Failed to resolve");
        assert!(addrs.next().is_some(), "Should return at least one IP");
    }

    #[tokio::test]
    async fn test_secure_resolver_failure() {
        let resolver =
            SecureResolver::new(vec!["invalid.domain.that.does.not.exist.test".to_string()]);
        let name = Name::from_str("example.com").unwrap();

        let result = resolver.resolve(name).await;
        assert!(result.is_err());
    }
}
