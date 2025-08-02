// Adicione estas dependências ao seu arquivo Cargo.toml:
// chrono = "0.4"
// rust_decimal = "1.35"
// rust_decimal-macros = "1.35"

use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Estrutura para os dados que precisam ser atualizados de forma atômica em conjunto.
/// Em C#, isso era protegido por um `lock`. Em Rust, colocamos os dados dentro de um `Mutex`.
#[derive(Debug, Default)]
struct MetricsState {
    total_requests: i64,
    total_duration_nanos: u128,
    total_data_size: i64,
    latency_buffer: Vec<f64>,
    latency_index: usize,
    latency_count: usize,
}

/// Corresponde à classe XMetrics.
/// É seguro para uso concorrente (`thread-safe`).
#[derive(Debug)]
pub struct XMetrics {
    state: Mutex<MetricsState>,
    total_errors: AtomicI64,
    start_instant: Instant,
}

/// Corresponde à classe PerformanceMetrics.
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub timestamp: Duration,
    pub total_requests: i64,
    pub total_errors: i64,
    pub requests_per_second: f64,
    pub average_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub data_size_kb: i64,
    pub data_per_second_kb: Decimal,
    pub success_rate: f64,
}

impl XMetrics {
    const LATENCY_BUFFER_SIZE: usize = 10_000;

    /// Cria uma nova instância de XMetrics. Corresponde ao construtor.
    pub fn new() -> Self {
        Self {
            state: Mutex::new(MetricsState {
                latency_buffer: vec![0.0; Self::LATENCY_BUFFER_SIZE],
                ..Default::default()
            }),
            total_errors: AtomicI64::new(0),
            start_instant: Instant::now(),
        }
    }

    /// Marca o início de uma operação.
    /// Retorna um `Instant` que deve ser passado para `end`.
    /// Isso corrige uma condição de corrida do código C# original.
    pub fn start(&self) -> Instant {
        Instant::now()
    }

    /// Registra um erro de forma atômica.
    pub fn error(&self) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Marca o fim de uma operação, registrando sua duração e o tamanho dos dados.
    pub fn end(&self, start_time: Instant, data_size: i64) {
        let duration = start_time.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;

        // Adquire o "lock" para atualizar o estado compartilhado.
        // O lock é liberado automaticamente no fim do escopo.
        let mut state = self.state.lock().unwrap();

        state.total_data_size += data_size;
        state.total_requests += 1;
        state.total_duration_nanos += duration.as_nanos();

        // Lógica do buffer circular de latência
        let index = state.latency_index % Self::LATENCY_BUFFER_SIZE;
        state.latency_buffer[index] = duration_ms;
        state.latency_index += 1;

        if state.latency_count < Self::LATENCY_BUFFER_SIZE {
            state.latency_count += 1;
        }
    }

    /// Gera um snapshot das métricas de performance atuais.
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let (total_requests, total_duration_nanos, total_data_size, mut latency_snapshot) = {
            let state = self.state.lock().unwrap();
            let count = state.latency_count;
            // Cria uma cópia (snapshot) dos dados de latência
            let snapshot = state.latency_buffer[..count].to_vec();
            (
                state.total_requests,
                state.total_duration_nanos,
                state.total_data_size,
                snapshot,
            )
        }; // O lock é liberado aqui.

        let total_errors = self.total_errors.load(Ordering::Relaxed);
        let elapsed_duration = self.start_instant.elapsed();
        let elapsed_seconds = elapsed_duration.as_secs_f64();

        let avg_duration_ms = if total_requests > 0 {
            (total_duration_nanos as f64 / 1_000_000.0) / total_requests as f64
        } else {
            0.0
        };

        latency_snapshot.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let requests_per_second = if elapsed_seconds > 0.0 {
            total_requests as f64 / elapsed_seconds
        } else {
            0.0
        };

        let success_rate = if total_requests > 0 {
            (total_requests - total_errors) as f64 / total_requests as f64 * 100.0
        } else {
            100.0
        };

        let data_size_kb = total_data_size / 1024;
        let data_per_second_kb = if elapsed_seconds > 0.0 {
            Decimal::from(total_data_size) / Decimal::from_f64(elapsed_seconds).unwrap_or(dec!(1.0)) / dec!(1024.0)
        } else {
            dec!(0.0)
        };

        PerformanceMetrics {
            timestamp: elapsed_duration,
            total_requests,
            total_errors,
            requests_per_second,
            average_latency_ms: avg_duration_ms,
            p50_latency_ms: Self::get_percentile(&latency_snapshot, 50.0),
            p95_latency_ms: Self::get_percentile(&latency_snapshot, 95.0),
            p99_latency_ms: Self::get_percentile(&latency_snapshot, 99.0),
            data_size_kb,
            data_per_second_kb,
            success_rate,
        }
    }

    /// Função auxiliar para calcular percentis.
    fn get_percentile(sorted_data: &[f64], percentile: f64) -> f64 {
        if sorted_data.is_empty() {
            return 0.0;
        }
        let index = (percentile / 100.0 * sorted_data.len() as f64).ceil() as usize - 1;
        sorted_data[index.clamp(0, sorted_data.len() - 1)]
    }
}

// Implementação do trait Display para formatar a saída, similar ao ToString() do C#.
impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Performance Metrics Report")?;
        writeln!(f, "========================")?;
        writeln!(f, "Timestamp: {:?}", self.timestamp)?;
        writeln!(f, "Total Data: {} KB", self.data_size_kb)?;
        writeln!(f, "Total Data/Second: {:.2} KB", self.data_per_second_kb)?;
        writeln!(f, "Total Requests: {}", self.total_requests)?;
        writeln!(f, "Total Errors: {}", self.total_errors)?;
        writeln!(f, "Requests Per Second: {:.0}", self.requests_per_second)?;
        writeln!(f, "Average Latency: {:.3} ms", self.average_latency_ms)?;
        writeln!(f, "P50 Latency: {:.3} ms", self.p50_latency_ms)?;
        writeln!(f, "P95 Latency: {:.3} ms", self.p95_latency_ms)?;
        writeln!(f, "P99 Latency: {:.3} ms", self.p99_latency_ms)?;
        writeln!(f, "Success Rate: {:.2}%", self.success_rate)?;
        writeln!(f, "\nGenerated at: {} UTC", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"))
    }
}