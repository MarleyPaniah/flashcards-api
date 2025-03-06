use password_worker::{BcryptConfig, PasswordWorker};
use tracing::debug;

pub async fn generate_password_hash(
    password: &String,
    cost: Option<u32>,
    max_threads: Option<usize>,
) -> Result<String, Box<dyn std::error::Error>> {
    let cost = cost.unwrap_or(12); // bcrypt cost value
    let max_threads = max_threads.unwrap_or(4); // rayon thread pool max threads
    let password_worker = PasswordWorker::new_bcrypt(max_threads)?;

    let hashed_password = password_worker
        .hash(password, BcryptConfig { cost })
        .await?;
    debug!("Hashed password: {:?}", hashed_password);

    // let is_valid = password_worker.verify(password, hashed_password).await?;
    // debug!("Verification result: {:?}", is_valid);

    Ok(hashed_password)
}
