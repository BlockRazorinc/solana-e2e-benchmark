#[derive(Debug, Clone, clap::Parser)]
pub struct RunArgs {
    #[clap(
        long = "yellow_stone_grpc_endpoint",
        default_value = "http://127.0.0.1:10001"
    )]
    pub yellow_stone_grpc_endpoint: String,

    #[clap(long = "shred_listen_addr", default_value = "0.0.0.0:20000")]
    pub shred_listen_addr: String,

    #[clap(long = "tx_sender_addr", default_value = "127.0.0.1:10000")]
    pub tx_sender_addr: String,

    #[clap(
        long = "sender_grpc_endpoint",
        default_value = "http://newyork.solana-grpc.blockrazor.xyz:80"
    )]
    pub sender_grpc_endpoint: String,

    #[clap(long = "web_port", default_value = "3000")]
    pub web_port: u32,

    #[clap(long = "concerned_account", default_value = "#")]
    pub concerned_account: String,

    #[clap(long = "sender_grpc_auth_key", default_value = "#")]
    pub sender_grpc_auth_key: String,
}
