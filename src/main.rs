extern crate mammut;
extern crate dotenv;
extern crate rand;

use mammut::{Registration, Mastodon, Data};
use mammut::status_builder::StatusBuilder;
use mammut::apps::{AppBuilder, Scope};
use dotenv::dotenv;
use rand::distributions::{IndependentSample, Range};

use std::env;
use std::io;
use std::thread::sleep;
use std::time::Duration;

const GOOD_WORDS: [&str; 11] = [
    "great",
    "fantastic",
    "the best",
    "The Best(tm)",
    "wonderful",
    "amazing",
    "so good",
    "cute",
    "a good girl",
    "a good fox",
    "on mastodon",
];

fn main_loop(mastodon: &Mastodon) -> mammut::Result<()> {
    loop {
        let between: Range<usize> = Range::new(0, 11);
        let mut rng = rand::thread_rng();

        let cow = format!(
            " ______________________
< @lottievixen@dev.glitch.social is {} >
 -------------------------------------
        \\   ^__^
         \\  (oo)\\_______
             (__)\\               )\\/\\
                     ||--------w |
                     ||            ||",
            GOOD_WORDS[between.ind_sample(&mut rng)]
        );
        let status = StatusBuilder::new(cow);
        mastodon.new_status(status)?;

        // one post every hour
        sleep(Duration::from_secs(60 * 60));
    }
}

fn run() -> mammut::Result<()> {
    dotenv().ok();

    let app = AppBuilder {
        client_name: "lottiebot",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scope::ReadWrite,
        website: None,
    };

    let mastodon = match env::var("ACCESS_TOKEN")
        .and_then(|key| {
            env::var("CLIENT_ID").map(|client_id| (key, client_id))
        })
        .and_then(|(key, client_id)| {
            env::var("CLIENT_SECRET").map(|client_secret| (key, client_id, client_secret))
        })
        .and_then(|(key, client_id, client_secret)| {
            env::var("REDIRECT").map(|redirect| (key, client_id, client_secret, redirect))
        }) {
        Ok((key, client_id, client_secret, redirect)) => {
            Mastodon::from_data(Data {
                base: "https://asonix.dog".into(),
                client_id: client_id.into(),
                client_secret: client_secret.into(),
                redirect: redirect.into(),
                access_token: key.into(),
            })
        }
        Err(e) => {
            let mut registration = Registration::new("https://asonix.dog");
            registration.register(app)?;

            println!("Error: {}", e);
            let url = registration.authorise()?;

            println!("Please open this URL in your browser: {}", url);
            println!("Please enter the authorization key you received from Mastodon: ");
            let mut buffer = String::new();
            let stdin = io::stdin();

            stdin.read_line(&mut buffer).unwrap();

            registration.create_access_token(buffer)?
        }
    };

    main_loop(&mastodon)?;

    Ok(())
}

fn main() {
    run().unwrap();
}
