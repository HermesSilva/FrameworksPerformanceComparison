mod XPayload;
mod XMetrics;
mod XPayloadGenerator;

use std::{env, sync::Arc, time::Duration};
use reqwest::Client;
use tokio::{sync::Semaphore, time::interval};
use futures::stream::{FuturesUnordered, StreamExt};

static METRICS: once_cell::sync::Lazy<Arc<XMetrics::XMetrics>> =
    once_cell::sync::Lazy::new(|| Arc::new(XMetrics::XMetrics::new()));
static PAYLOADS: once_cell::sync::Lazy<Vec<XPayload::XPayload>> =
    once_cell::sync::Lazy::new(|| XPayloadGenerator::XPayloadGenerator::create_multiple_payloads(10_000));

const BATCH: usize = 50;

#[tokio::main(flavor = "multi_thread", worker_threads = 128)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let clients: usize = args.get(32).and_then(|v| v.parse().ok()).unwrap_or(32);

    println!("Size: {}", clients);

    // Início do monitor de métricas
    let metrics = METRICS.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            clear_console();
            println!("{}", metrics.GetMetrics());
        }
    });

    // Disparar workers
    let semaphore = Arc::new(Semaphore::new(clients));
    for i in 0..clients {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        tokio::spawn(async move {
            //tokio::time::sleep(Duration::from_millis((i * 10) as u64)).await;
            run_client(permit).await;
        });
        println!("{}", i);
    }

    println!("Pressione CTRL+C para encerrar...");
    tokio::signal::ctrl_c().await.unwrap();
}

fn clear_console() {
    #[cfg(windows)]
    std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();

    #[cfg(not(windows))]
    std::process::Command::new("clear").status().unwrap();
}

async fn run_client(_permit: tokio::sync::OwnedSemaphorePermit) {
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(200)
        .http2_adaptive_window(true)
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap();

    let mut pl_index = 0;
    //let mut buffer = Vec::with_capacity(4096);
    //  let mut futs = FuturesUnordered::new();

    for _ in 0..1_000_000_000 {
        if pl_index >= PAYLOADS.len() {
            pl_index = 0;
        }
        // let payload = PAYLOADS[pl_index].clone();
        // pl_index += 1;
        //
        // buffer.clear();
        // serde_json::to_writer(&mut buffer, &payload).unwrap();
        //let body = buffer.clone();

        //let client_ref = client.clone();
        METRICS.Start();

        // let fut = async move {
            // let resp = client_ref
            //     .post("http://localhost:21012/")
            //     .header("Content-Type", "application/json")
            //     .body(body)
            //     .send()
            //     .await;
            //
            // match resp {
            //     Ok(r) if r.status().is_success() => {}
            //     Ok(r) => {
            //         METRICS.Error();
            //         eprintln!("HTTP inválido: {}", r.status());
            //     }
            //     Err(e) => {
            //         METRICS.Error();
            //         eprintln!("Erro: {}", e);
            //     }
            // }

            METRICS.End(0);
        // };

         // futs.push(fut);

        // Controla o paralelismo real: só avança se abaixo do BATCH
         //if futs.len() >= BATCH {
         //     futs.next().await;
         //}
    }

    // Aguarda os pendentes restantes
    //  while futs.next().await.is_some() {}
}
