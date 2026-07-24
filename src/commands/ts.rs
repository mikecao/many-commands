use clap::Args;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Args)]
pub struct TsArgs {
    /// Print the timestamp in milliseconds instead of seconds
    #[arg(long)]
    pub millis: bool,
}

pub fn run(args: TsArgs) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the unix epoch");

    if args.millis {
        println!("{}", now.as_millis());
    } else {
        println!("{}", now.as_secs());
    }
}
