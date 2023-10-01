use std::{thread, time::Duration};

use anyhow::{bail, Result};
use pushover_rs::{send_pushover_request, MessageBuilder};
use structopt::StructOpt;

// This is meant to be ran as an SMF transient service so returning OK is fine.
const SMF_EXIT_OK: i32 = 0;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, required = true)]
    user_key: String,
    #[structopt(short, long, required = true)]
    application_token: String,
    #[structopt(short, long, required = true)]
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let message = MessageBuilder::new(
        &opt.user_key,
        &opt.application_token,
        &opt.message,
    )
    .set_priority(1)
    .build();

    loop {
        match send_pushover_request(message.clone()).await {
            // Pushover response received
            Ok(pr) => match pr.status {
                1 => {
                    println!("Pushover sent successfully... exiting");
                    break;
                }
                // Pushover response was an error:
                // - Invalid user key?
                // - Invalid application token?
                _ => {
                    bail!("Pushover request failed: {:#?}", pr.errors);
                }
            },
            // HTTP error
            Err(e) => {
                // We just want to log this error and keep trying our request
                // until we get a successful response from the Pushover server.
                eprintln!("Failed to send pushover request: {e}");
            }
        }

        thread::sleep(Duration::from_secs(5));
    }

    std::process::exit(SMF_EXIT_OK)
}
