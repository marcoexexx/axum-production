pub use self::error::{Error, Result};

mod error;
pub mod routes_login;
pub mod routes_static;

pub const AUTH_TOKEN: &str = "auth-token";
