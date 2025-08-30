use password_worker::{BcryptConfig, PasswordWorker};
use tracing::debug;

use crate::config::Config;

/**
Generate a password hash from a password string.
 */
pub async fn generate_password_hash(
    config: &Config,
    password: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Generating password hash...");

    // bcrypt cost value
    let cost = config.password_hash_gen_cost as u32;

    // rayon thread pool max threads
    let max_threads: usize = config.password_hash_gen_max_threads;

    let password_worker = PasswordWorker::new_bcrypt(max_threads)?;

    let hashed_password = password_worker
        .hash(password, BcryptConfig { cost })
        .await?;

    Ok(hashed_password)
}
