pub mod jwt;
pub mod oauth2;
pub mod middleware;
pub mod client;
pub mod error;

pub use jwt::{JwtClaims, JwtKeys, TokenGenerator, TokenValidator};
pub use oauth2::{OAuth2Config, ClientCredentials, AccessToken, TokenResponse};
pub use error::{AuthError, Result};

#[cfg(feature = "server")]
pub use middleware::AuthLayer;

#[cfg(feature = "db")]
pub use client::ClientStore;
