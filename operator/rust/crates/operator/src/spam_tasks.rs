#![allow(missing_docs)]
use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use dotenv::dotenv;
use eigen_logging::{get_logger, init_logger, log_level::LogLevel};
use eigen_utils::get_signer;
use eyre::Result;
use trappist_utils::{trappistservicemanager::trappistServiceManager, trappistData};
use once_cell::sync::Lazy;
use rand::Rng;
use std::{env, str::FromStr};
use tokio::time::{self, Duration};

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

#[allow(unused)]
static KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("failed to retrieve private key"));

#[allow(unused)]
/// Generate random task names from the given adjectives and nouns
fn generate_random_name() -> String {
    let adjectives = ["Quick", "Lazy", "Sleepy", "Noisy", "Hungry"];
    let nouns = ["Fox", "Dog", "Cat", "Mouse", "Bear"];

    let mut rng = rand::thread_rng();

    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];
    let number: u16 = rng.gen_range(0..1000);

    format!("{}{}{}", adjective, noun, number)
}

#[allow(unused)]
/// Calls CreateNewTask function of the Trappist service manager contract
async fn create_new_task(task_name: &str) -> Result<()> {
    let data = std::fs::read_to_string("contracts/deployments/trappist/31337.json")?;
    let parsed: trappistData = serde_json::from_str(&data)?;
    let trappist_contract_address: Address =
        parsed.addresses.trappist_service_manager.parse()?;
    let pr = get_signer(&KEY.clone(), ANVIL_RPC_URL);
    let signer = PrivateKeySigner::from_str(&KEY.clone())?;
    let trappist_contract = trappistServiceManager::new(trappist_contract_address, pr);

    let tx = trappist_contract
        .createNewTask(task_name.to_string())
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Transaction successfull with tx : {:?}",
        tx.transaction_hash
    );

    Ok(())
}

#[allow(unused)]
/// Start creating tasks at every 15 seconds
async fn start_creating_tasks() {
    let mut interval = time::interval(Duration::from_secs(15));
    init_logger(LogLevel::Info);
    loop {
        interval.tick().await;
        let random_name = generate_random_name();
        get_logger().info(
            &format!("Creating new task with name: {} ", random_name),
            &"start_creating_tasks",
        );
        let _ = create_new_task(&random_name).await;
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();
    start_creating_tasks().await;
}
