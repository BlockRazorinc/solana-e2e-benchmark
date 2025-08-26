use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use clap::Parser;
use serde::Serialize;
use serde_json::json;
use solanabench::common::types::RunArgs;
use solanabench::common::utils::now_ts_iso8601;
use solanabench::common::yellow_stone_grpc_client::CHANNEL;
use solanabench::common::{trader_simulator, tx_constructor, tx_sender, yellow_stone_grpc_client};
use tracing::info;

async fn init_app(port: u32) {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/send_tx", get(call_send_tx_wrapper));

    let addr = format!("0.0.0.0:{}", port);
    println!("Server running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let arg = RunArgs::parse();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let port = arg.web_port;
    info!("sender grpc endpoint: {}", arg.sender_grpc_endpoint);
    let app = tokio::spawn(init_app(port));

    let run_args = RunArgs::parse();
    let sender_grpc_endpoint = run_args.sender_grpc_endpoint;
    let sender_grpc_auth_key = run_args.sender_grpc_auth_key;
    let _ = tx_sender::init_client(sender_grpc_endpoint, sender_grpc_auth_key).await;
    tokio::spawn(async move {
        yellow_stone_grpc_client::grpc_subscribe(
            run_args.yellow_stone_grpc_endpoint,
            run_args.concerned_account,
        )
        .await
    });

    app.await.unwrap();
}

async fn monitor(signature: String) -> String {
    let mut rx = CHANNEL.1.clone();
    while rx.changed().await.is_ok() {
        let msg = rx.borrow().clone(); // String
        if msg.contains(&signature) {
            let ts = now_ts_iso8601();
            return ts;
        }
    }
    json!({"error": "signature not found"}).to_string()
}

async fn send_tx() -> String {
    let construct_start = now_ts_iso8601();
    info!("Begin construct transaction");

    let (sig, tx_str) = tx_constructor::construct_tx().await;

    let construct_end = now_ts_iso8601();
    info!("End construct transaction, signature: {}", sig);

    let monitor_task = tokio::spawn(monitor(sig.clone()));

    let send_start = now_ts_iso8601();
    let _ = trader_simulator::one_client_test(sig.clone(), tx_str).await;
    let send_end = now_ts_iso8601();

    let grpc_monitor_result = monitor_task.await.unwrap();

    let result = json!({
        "signature": sig,
        "construct_start": construct_start,
        "construct_end": construct_end,
        "send_start": send_start,
        "send_end": send_end,
        "grpc_end": grpc_monitor_result,
    });

    result.to_string()
}

#[derive(Serialize)]
struct TxResponse {
    message: String,
}

async fn call_send_tx_wrapper() -> impl IntoResponse {
    let msg = send_tx().await;
    let body = TxResponse { message: msg };
    Json(body)
}

async fn index() -> impl IntoResponse {
    info!("get index");
    Html(
        r#"
 <!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Sender Test </title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      body { font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial;
             max-width: 720px; margin: 40px auto; padding: 0 16px; }
      button, select { padding: 10px 16px; border-radius: 12px; border: 1px solid #ccc;
               cursor: pointer; font-size: 16px; margin-right: 8px; }
      pre, textarea { background: #f6f8fa; padding: 12px; border-radius: 12px; overflow: auto; }
      .row { margin: 12px 0; }
      textarea { width: 100%; height: 200px; resize: none; margin-bottom: 12px; }
    </style>
  </head>
  <body>
    <h1>Send Tx Tester</h1>
    <div class="row">
      <button id="btn">Click me to call /api/send_tx</button>
      <select id="count">
        <option value="1" selected>1 time</option>
        <option value="2">2 time</option>
        <option value="5">5 times</option>
        <option value="10">10 times</option>
        <option value="15">15 times</option>
        <option value="20">20 times</option>
        <option value="33">33 times</option>
        <option value="100">100 times</option>
      </select>
      <button id="auto">Auto Run</button>
    </div>

    <div class="row">
      <strong>Request sent at (client time, μs precision):</strong>
      <div id="t-send">-</div>
    </div>
    <div class="row">
      <strong>Response received at (client time, μs precision):</strong>
      <div id="t-recv">-</div>
    </div>

    <div class="row">
      <strong>Server response (with client timing):</strong>
      <pre id="output">-</pre>
    </div>

    <div class="row">
      <strong>Auto Run Log:</strong><br/>
      <div id="logs-container"></div>
    </div>

    <script>
      const $ = (id) => document.getElementById(id);

      function formatWithMicros(epochMs, perfMicros) {
        const date = new Date(epochMs);
        const iso = date.toISOString();
        const micros = Math.floor((perfMicros % 1) * 1000);
        return iso.replace("Z", "") + "." + micros.toString().padStart(3, "0") + "Z";
      }

      async function callApiOnce() {
        const t0 = performance.now();
        const epochSend = Date.now();
        $("t-send").textContent = formatWithMicros(epochSend, t0);
        $("t-recv").textContent = "(waiting for response...)";
        $("output").textContent = "(loading...)";

        try {
          const res = await fetch("/api/send_tx", { method: "GET" });
          const t1 = performance.now();
          const epochRecv = Date.now();
          $("t-recv").textContent = formatWithMicros(epochRecv, t1);

          if (!res.ok) {
            $("output").textContent = `HTTP ${res.status}`;
            return `HTTP ${res.status}`;
          }
          const data = await res.json();

          data.client_request_time = formatWithMicros(epochSend, t0);
          data.client_response_time = formatWithMicros(epochRecv, t1);

          $("output").textContent = JSON.stringify(data, null, 2);
          return JSON.stringify(data);
        } catch (e) {
          const t1 = performance.now();
          const epochRecv = Date.now();
          $("t-recv").textContent = formatWithMicros(epochRecv, t1);
          $("output").textContent = "Request failed: " + (e && e.message ? e.message : e);
          return "Request failed: " + (e && e.message ? e.message : e);
        }
      }

      $("btn").addEventListener("click", callApiOnce);

      $("auto").addEventListener("click", async () => {
        const total = parseInt($("count").value, 10);

        const container = $("logs-container");
        const newLog = document.createElement("textarea");
        newLog.readOnly = true;
        newLog.rows = 10;
        newLog.placeholder = `Auto Run at ${new Date().toISOString()}`;
        container.insertBefore(newLog, container.firstChild);

        for (let i = 1; i <= total; i++) {
          const result = await callApiOnce();
          newLog.value += `[Run ${i}] ${result}\n`;

          if (i < total) {
            await new Promise((resolve) => setTimeout(resolve, 5000));
          }
        }
      });
    </script>
  </body>
</html>

"#,
    )
}
