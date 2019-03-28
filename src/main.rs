#[macro_use]
extern crate serde_derive;

use reqwest::Client;
use std::error::Error;
use std::time::{Duration, SystemTime};

type Result<T = ()> = std::result::Result<T, Box<Error>>;

#[derive(Serialize, Deserialize, PartialEq)]
enum LockState {
    Locked,
    Unlocked,
}

#[derive(Deserialize)]
struct CastleGetData {
    state: LockState,
    last_change: u64,
}

#[derive(Serialize)]
struct CastlePutData {
    state: LockState,
}

#[derive(Deserialize)]
struct Config {
    client: ClientConfig,
    policy: PolicyConfig,
}

#[derive(Deserialize)]
struct ClientConfig {
    endpoint: String,
}

#[derive(Deserialize)]
struct PolicyConfig {
    lock_after_seconds: u64,
}

fn main() -> Result {
    run(&read_config()?)
}

fn read_config() -> Result<Config> {
    let conf_str = std::fs::read_to_string("baz-out.toml")?;
    toml::from_str(&conf_str).map_err(From::from)
}

fn run(config: &Config) -> Result {
    let client = &Client::new();
    let endpoint = &config.client.endpoint;
    let lock_after_duration = Duration::from_secs(config.policy.lock_after_seconds);
    loop {
        let resp = get_data(client, endpoint)?;
        match resp.state {
            LockState::Unlocked => {
                let last_change = std::time::UNIX_EPOCH + Duration::from_secs(resp.last_change);
                let passed = SystemTime::now().duration_since(last_change)?;
                if lock_after_duration <= passed {
                    lock(client, endpoint)?;
                    std::thread::sleep(lock_after_duration);
                } else {
                    std::thread::sleep(lock_after_duration - passed);
                }
            }
            LockState::Locked => {
                std::thread::sleep(lock_after_duration);
            }
        }
    }
}

fn lock(client: &Client, endpoint: &str) -> Result {
    put_data(
        client,
        endpoint,
        &CastlePutData {
            state: LockState::Locked,
        },
    )
}

fn get_data(client: &Client, endpoint: &str) -> Result<CastleGetData> {
    let resp_str = client.get(endpoint).send()?.text()?;
    Ok(serde_json::from_str(&resp_str)?)
}

fn put_data(client: &Client, endpoint: &str, data: &CastlePutData) -> Result {
    let req_str = serde_json::to_string(data)?;
    client.put(endpoint).body(req_str).send()?;
    Ok(())
}
