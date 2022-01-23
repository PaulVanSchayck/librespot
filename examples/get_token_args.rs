use std::env;

use librespot::core::config::SessionConfig;
use librespot::core::keymaster;
use librespot::core::session::Session;
use librespot::core::authentication::Credentials;
use librespot::protocol::authentication::AuthenticationType;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} USERNAME AUTH_DATA CLIENT_ID", args[0]);
        return;
    }

    let credentials = Credentials {
        username: String::from(&args[1]),
        auth_type: AuthenticationType::AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS,
        auth_data: base64::decode(String::from(&args[2])).unwrap(),
    };

    let session = Session::connect(session_config, credentials, None)
        .await
        .unwrap();

    println!(
        "{}",
        keymaster::get_token(&session, &args[3], SCOPES)
            .await
            .unwrap()
            .access_token
    );
}
