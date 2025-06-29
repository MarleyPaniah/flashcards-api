use std::env;

use password_worker::{BcryptConfig, PasswordWorker};
use tracing::{debug, trace};

/**
Generate a password hash from a password string.

TODO: Env variables shouldn't be read every time this function is called.
The code reading them should be moved to a shared state.
 */
pub async fn generate_password_hash(
    password: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Generating password hash...");

    // bcrypt cost value
    let cost: u32 = env::var("PWD_HASH_GEN__COST")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(12);

    // rayon thread pool max threads
    let max_threads: usize = env::var("PWD_HASH_GEN__MAX_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(4);

    let password_worker = PasswordWorker::new_bcrypt(max_threads)?;

    let hashed_password = password_worker
        .hash(password, BcryptConfig { cost })
        .await?;
    trace!("Hashed password: {:?}", hashed_password);

    // let is_valid = password_worker.verify(password, hashed_password).await?;
    // debug!("Verification result: {:?}", is_valid);

    Ok(hashed_password)
}
