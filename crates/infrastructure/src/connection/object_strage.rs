use cf_r2_sdk::{builder::Builder, operator::Operator};
use domain::value_object::error::connection::ConnectionError;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub(super) async fn connect_r2() -> Result<Operator, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static CLOUDFLARE_R2_BUCKET_NAME: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_URI_ENDPOINT: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = CLOUDFLARE_R2_BUCKET_NAME.set(env::var("CLOUDFLARE_R2_BUCKET_NAME")?);
    let _ = CLOUDFLARE_R2_URI_ENDPOINT.set(env::var("CLOUDFLARE_R2_URI_ENDPOINT")?);
    let _ = CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID
        .set(env::var("CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID")?);
    let _ = CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY
        .set(env::var("CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY")?);
    // create Operator instance
    Ok(Builder::new()
        .set_bucket_name(
            CLOUDFLARE_R2_BUCKET_NAME
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "CLOUDFLARE_R2_BUCKET_NAME".to_string(),
                ))?
                .to_owned(),
        )
        .set_endpoint(
            CLOUDFLARE_R2_URI_ENDPOINT
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "CLOUDFLARE_R2_URI_ENDPOINT".to_string(),
                ))?
                .to_owned(),
        )
        .set_access_key_id(
            CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID".to_string(),
                ))?
                .to_owned(),
        )
        .set_secret_access_key(
            CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY".to_string(),
                ))?
                .to_owned(),
        )
        .create_client())
}
