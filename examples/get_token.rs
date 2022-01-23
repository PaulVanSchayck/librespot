use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use librespot::core::config::SessionConfig;
use librespot::core::keymaster;
use librespot::core::session::Session;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} CREDENTIALS_CACHE CLIENT_ID", args[0]);
        return;
    }

    let cache =
        librespot::core::cache::Cache::new(Some(PathBuf::from_str(&args[1]).unwrap()), None, None, None)
            .expect("Could not create librespot cache");

    let credentials = cache.credentials().expect("No credentials present");

    let session = Session::connect(session_config, credentials, None)
        .await
        .unwrap();

    println!(
        "{}",
        keymaster::get_token(&session, &args[2], SCOPES)
            .await
            .unwrap()
            .access_token
    );
}
