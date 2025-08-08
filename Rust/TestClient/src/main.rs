use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tokio::time::{sleep, Duration};
use futures::future::join_all; // ✅ Importe explicitamente

const NUM_THREADS: usize = 10;

static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    println!("🚀 Usando {} threads", NUM_THREADS);

    tokio::spawn(monitor_rps());

    let handles: Vec<_> = (0..NUM_THREADS)
        .into_par_iter()
        .map(|_| tokio::spawn(worker()))
        .collect();

    join_all(handles).await; // ✅ Sem prefixo completo
}

async fn worker() {
    let mut local_count = 0u64;
    loop {
        sleep(Duration::from_micros(100)).await;
        local_count += 1;
        if local_count % 1000 == 0 {
            TOTAL_REQUESTS.fetch_add(local_count, Ordering::Relaxed);
            local_count = 0;
        }
    }
}

async fn monitor_rps() {
    let pt_br = Locale::pt; // ✅ sem parênteses
    let start = Instant::now();

    loop {
        sleep(Duration::from_secs(1)).await;
        let elapsed = start.elapsed().as_secs().max(1);
        let total = TOTAL_REQUESTS.load(Ordering::Relaxed);
        let rps = total as f64 / elapsed as f64;

        print!("{esc}c", esc = 27 as char);
        println!("📊 Estatísticas de Desempenho");
        println!("----------------------------");
        println!("Tempo: {}s THDs: {}", elapsed, NUM_THREADS);
        println!("Total de requisições: {}", total.to_formatted_string(&pt_br));
        println!("Requisições por segundo: {}", (rps as i64).to_formatted_string(&pt_br));
    }
}


/*
use num_format::{Locale, ToFormattedString};
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{sleep, Duration};

// 🔢 Contadores globais
static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);
static START_TIMESTAMP: std::sync::LazyLock<Instant> = std::sync::LazyLock::new(Instant::now);
static num_threads: usize = 10;

#[tokio::main]
async fn main() {
    // 🔧 CONFIGURAÇÃO: Defina quantas threads nativas usar

    // 🧱 Cria um pool de threads controlado (ex: 8 THDs)
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .thread_name(|i| format!("worker-{}", i))
        .build()
        .expect("Falha ao criar pool de threads");

    println!("🚀 Iniciando com {} threads nativas", num_threads);
    println!("💡 Cada thread rodará centenas de tasks assíncronas...");

    // 🔁 Usa o pool para iniciar o trabalho concorrente
    thread_pool.install(|| {
        // 📊 Inicia monitoramento de RPS
        std::thread::spawn(monitor_rps);

        // 🚀 Inicia múltiplas tasks assíncronas (ex: 1000) em background
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut handles = vec![];
            for _ in 0..1000 {
                let handle = tokio::spawn(worker());
                handles.push(handle);
            }

            // Aguarda todas (infinitamente)
            futures::future::join_all(handles).await;
        });
    });

    // ✅ Espera sinal para encerrar
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n👋 Encerrando...");
}

// 🛠️ Worker: simula uma interação concorrente (pode ser HTTP, DB, etc)
async fn worker() {
    loop {
        // 🔁 Simula um pequeno processamento (ex: I/O, cálculo)
        sleep(Duration::from_micros(100)).await; // Ajuste para testar limite

        // 📈 Incrementa contador global (muito rápido)
        TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed);
    }
}

// 📊 Monitora RPS a cada segundo
fn monitor_rps() {
    let pt_br = Locale::pt;
    loop {
        std::thread::sleep(Duration::from_secs(1));

        let elapsed = START_TIMESTAMP.elapsed().as_secs().max(1);
        let total = TOTAL_REQUESTS.load(Ordering::Relaxed);
        let rps = total as f64 / elapsed as f64;

        // 🧹 Limpa console e mostra estatísticas
        print!("{esc}c", esc = 27 as char);
        println!("📊 Estatísticas de Desempenho");
        println!("----------------------------");
        println!("Tempo: {}s, Threads {}", elapsed,num_threads);
        println!("Total de requisições: {}", total.to_formatted_string(&pt_br));
        println!("Requisições por segundo: {}", (rps as i64).to_formatted_string(&pt_br));
    }
}
*/

/*

use num_format::{Locale, ToFormattedString};
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::sleep;
use std::{env, sync::Arc, sync::LazyLock, time::{Duration, Instant},};
static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);
static START_TIMESTAMP: LazyLock<Instant> = LazyLock::new(|| Instant::now());



#[tokio::main]
async fn main() {
    let num_threads = 10;

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    pool.install(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(expensive_computation());
    });
    println!("Pressione CTRL+C para encerrar...");
    tokio::signal::ctrl_c().await.unwrap();
}

 async fn expensive_computation() {
    loop {
        sleep(Duration::from_micros(10)).await;
        let tr= TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed);
        if tr % 100 == 0
        {
            clear_console();
            let pt_br = Locale::pt;
            let elapsed_secs = START_TIMESTAMP.elapsed().as_secs().max(1);
            let requests_per_sec =
                TOTAL_REQUESTS.load(Ordering::Relaxed) as f64 / elapsed_secs as f64;
            let requests_per_second_fmt = (requests_per_sec as i64).to_formatted_string(&pt_br);
            println!("{}", requests_per_second_fmt);
        }
    }
}
 */

/*
mod XMetrics;
mod XPayload;
mod XPayloadGenerator;

use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use std::{env, sync::Arc, time::{Duration, Instant}, sync::LazyLock};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{sync::Semaphore, time::interval};
use num_format::{Locale, ToFormattedString}; // Dependência para formatação de números

static METRICS: once_cell::sync::Lazy<Arc<XMetrics::XMetrics>> =
    once_cell::sync::Lazy::new(|| Arc::new(XMetrics::XMetrics::new()));
static PAYLOADS: once_cell::sync::Lazy<Vec<XPayload::XPayload>> =
    once_cell::sync::Lazy::new(|| {
        XPayloadGenerator::XPayloadGenerator::create_multiple_payloads(10_000)
    });
static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);
static START_TIMESTAMP: LazyLock<Instant> = LazyLock::new(|| Instant::now());
#[tokio::main(flavor = "multi_thread", worker_threads = 128)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let clients = 30; //: usize = args.get(32).and_then(|v| v.parse().ok()).unwrap_or(32);

    println!("Size: {}", clients);

    // Início do monitor de métricas
    let metrics = METRICS.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            clear_console();
            let pt_br = Locale::pt;
            let elapsed_secs = START_TIMESTAMP.elapsed().as_secs().max(1);
            let requests_per_sec = TOTAL_REQUESTS.load(Ordering::Relaxed) as f64 / elapsed_secs as f64;
            let requests_per_second_fmt = (requests_per_sec as i64).to_formatted_string(&pt_br);
            println!("{}", requests_per_second_fmt);

            //println!("{}", metrics.GetMetrics());
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
    std::process::Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .unwrap();

    #[cfg(not(windows))]
    std::process::Command::new("clear").status().unwrap();
}

async fn run_client(_permit: tokio::sync::OwnedSemaphorePermit) {
    // let client = Client::builder()
    //     .pool_idle_timeout(Duration::from_secs(90))
    //     .pool_max_idle_per_host(200)
    //     .http2_adaptive_window(true)
    //     .tcp_keepalive(Some(Duration::from_secs(60)))
    //     .timeout(Duration::from_secs(20))
    //     .build()
    //     .unwrap();

    let mut pl_index = 0;
    // let mut buffer = Vec::with_capacity(4096);

    for _ in 0..2_000_000_000 {
        // if pl_index >= PAYLOADS.len() {
        //     pl_index = 0;
        // }
        // let payload = PAYLOADS[pl_index].clone();
        // pl_index += 1;
        // //
        // buffer.clear();
        // serde_json::to_writer(&mut buffer, &payload).unwrap();
        // let body = buffer.clone();

        //let client_ref = client.clone();
        //METRICS.Start();
        TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed);

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

        //METRICS.End(0);
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
*/

// fn clear_console() {
//     #[cfg(windows)]
//     std::process::Command::new("cmd")
//         .args(["/C", "cls"])
//         .status()
//         .unwrap();
//
//     #[cfg(not(windows))]
//     std::process::Command::new("clear").status().unwrap();
// }
