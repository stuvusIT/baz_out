#[macro_use]
extern crate serde_derive;

use reqwest::Client;
use std::time::{Duration, SystemTime};
use std::error::Error;

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
struct CastlePostData {
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
    let conf_str = std::fs::read_to_string("foo-out.toml")?;
    toml::from_str(&conf_str).map_err(From::from)
}

fn run(config: &Config) -> Result {
    let client = Client::new();
    loop {
        let lock_url = &config.client.endpoint;
        let resp_str = client
            .get(lock_url)
            .send()?
            .text()?;
        let resp : CastleGetData = serde_json::from_str(&resp_str)?;
        if resp.state == LockState::Unlocked {
            let last_change = std::time::UNIX_EPOCH + Duration::from_secs(resp.last_change);
            let passed = SystemTime::now().duration_since(last_change)?;
            if passed > Duration::from_secs(config.policy.lock_after_seconds) {
                let req = CastlePostData {
                    state: LockState::Locked
                };
                let req_str = serde_json::to_string(&req)?;
                client.put(lock_url).body(req_str).send()?;
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
