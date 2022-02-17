use std::net::SocketAddr;

use structopt::StructOpt;
// use tmelcrypt::Ed25519SK;

#[derive(Debug, StructOpt, Clone)]
pub struct CmdOpts {
    #[structopt(long, default_value = "127.0.0.1:11773")]
    /// Wallet API endpoint. For example localhost:11773
    pub daemon: SocketAddr,

    #[structopt(long, default_value = "__melminter_")]
    /// Prefixes for the "owned" wallets created by the melminter.
    pub wallet_prefix: String,

    #[structopt(long)]
    /// Which wallet to "draw" from when the melminter runs out of fee-paying mels.
    pub backup_wallet: String,

    #[structopt(long)]
    /// Force a certain number of threads. Defaults to the number of *physical* CPUs.
    pub threads: Option<usize>,
}
