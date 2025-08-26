# Solana E2E Benchmark

This project aims to test and analyze the end-to-end latency of Solana transaction.

## How to Run

### Getting Started

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/BlockRazorinc/solana-bench
    cd solana-bench
    ```

2.  **Implement Transaction Generation:**

    Implement the transaction generation logic in `src/common/tx_constructor.rs`. Ensure that the transaction signing account is the same as the `concerned_account` in step 4.

3.  **Install dependencies:**

    ```bash
    cargo build --release
    ```

4.  **Run the E2E Test:**

    You can run the main program (`src/BotSimulator/main.rs`) with the following parameters:

    ```bash
    ./target/release/BotSimulator \
        --yellow_stone_grpc_endpoint <YELLOW_STONE_GRPC_ENDPOINT> \
        --shred_listen_addr <SHRED_LISTEN_ADDR> \
        --sender_grpc_endpoint <SENDER_GRPC_ENDPOINT> \
        --sender_grpc_auth_key <SENDER_GRPC_AUTH_KEY> \
        --concerned_account <CONCERNED_ACCOUNT>
    ```

    **Parameter Descriptions:**

    * `--yellow_stone_grpc_endpoint`: The endpoint for the Yellow Stone gRPC service. (Default: `http://127.0.0.1:10001`)
    * `--shred_listen_addr`: The address to listen for shreds on. (Default: `0.0.0.0:20000`)
    * `--sender_grpc_endpoint`: The endpoint for the transaction sender gRPC service. (Default: `http://newyork.solana-grpc.blockrazor.xyz:80`)
    * `--sender_grpc_auth_key`: The authentication key for the transaction sender gRPC service. (Default: `#`)
    * `--concerned_account`: The Solana account address of the relevant transactions you want to monitor. (Default: `#`)

5.  **Run the Web Simulator:**

    If you want to use the web interface for client-side simulation testing, you can run `src/BotWebServer/main.rs`:

    ```bash
    ./target/release/BotWebServer \
        --web_port <WEB_PORT> \
        --sender_grpc_endpoint <SENDER_GRPC_ENDPOINT> \
        --sender_grpc_auth_key <SENDER_GRPC_AUTH_KEY> \
        --yellow_stone_grpc_endpoint <YELLOW_STONE_GRPC_ENDPOINT> \
        --concerned_account <CONCERNED_ACCOUNT>
    ```

    **Parameter Descriptions:**

    * `--web_port`: The port for the web server to listen on. (Default: `3000`)
    * Other parameters are the same as the main program.

    After starting, you can access `http://localhost:<WEB_PORT>` in your browser to interact with the application.

6.  **Observe Logs and Analyze Latency**


# Solana E2E Benchmark

该项目旨在测试和分析Solana交易端到端的全链路延迟。

## 如何运行

### 启动

1.  **克隆仓库:**
    ```bash
    git clone https://github.com/BlockRazorinc/solana-bench
    cd solana-bench
    ```

2. **安装依赖**
 
   实现`src/commom/tx_constructor.rs`的生成交易的代码，交易签发账户和第4步的`concerned_account`保持一致。
 
3. **安装依赖**
    ```bash
    cargo build --release
    ```

4.  **运行E2E测试:**

    您可以使用以下参数运行主程序 (`src/BotSimulator/main.rs`)：

    ```bash
    ./target/release/BotSimulator \
        --yellow_stone_grpc_endpoint <YELLOW_STONE_GRPC_ENDPOINT> \
        --shred_listen_addr <SHRED_LISTEN_ADDR> \
        --sender_grpc_endpoint <SENDER_GRPC_ENDPOINT> \
        --sender_grpc_auth_key <SENDER_GRPC_AUTH_KEY> \
        --concerned_account <CONCERNED_ACCOUNT>
    ```

    **参数说明:**

    *   `--yellow_stone_grpc_endpoint`: Yellow Stone gRPC 服务的端点。 (默认值: `http://127.0.0.1:10001`)
    *   `--shred_listen_addr`: 监听 shred 的地址。 (默认值: `0.0.0.0:20000`)
    *   `--sender_grpc_endpoint`: 交易发送器 gRPC 服务的端点。 (默认值: `http://newyork.solana-grpc.blockrazor.xyz:80`)
    *   `--sender_grpc_auth_key`: 交易发送器 gRPC 服务的认证密钥。 (默认值: `#`)
    *   `--concerned_account`: 您想要监控的相关交易 Solana 账户地址。 (默认值: `#`)

5.  **运行 Web 模拟**

    如果您想使用 Web 界面进行客户端的模拟测试，可以运行 `src/BotWebServer/main.rs`：

    ```bash
    ./target/release/BotWebServer \
        --web_port <WEB_PORT> \
        --sender_grpc_endpoint <SENDER_GRPC_ENDPOINT> \
        --sender_grpc_auth_key <SENDER_GRPC_AUTH_KEY> \
        --yellow_stone_grpc_endpoint <YELLOW_STONE_GRPC_ENDPOINT> \
        --concerned_account <CONCERNED_ACCOUNT>
    ```

    **参数说明:**

    *   `--web_port`: Web 服务器监听的端口。 (默认值: `3000`)
    *   其他参数与主程序相同。

    启动后，您可以在浏览器中访问 `http://localhost:<WEB_PORT>` 来与应用程序交互。

6.  **观察日志，分析延迟**
