use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::fmt;
use std::cell::RefCell;
use std::thread::{self, ThreadId};
use hdrhistogram::Histogram;
use dashmap::DashMap; // Para gerenciar histogramas por thread
use num_format::{Locale, ToFormattedString}; // Dependência para formatação de números

// O timer de início permanece como uma variável local da thread. É a abordagem mais rápida.
thread_local! {
    static CURRENT_START_TIME: RefCell<Option<Instant>> = RefCell::new(None);
}

/// Estrutura de métricas otimizada para performance extrema.
/// Elimina toda a contenção de locks no caminho quente (`end`) usando
/// histogramas locais por thread.
pub struct XMetrics {
    total_requests: AtomicU64,
    total_errors: AtomicU64,
    total_duration_nanos: AtomicU64,
    total_data_size: AtomicU64,
    start_timestamp: Instant,

    // Usamos um DashMap para associar um ThreadId a um Histograma.
    // Cada thread que executa uma operação terá seu próprio histograma,
    // eliminando a necessidade de locks compartilhados ao registrar latências.
    per_thread_histograms: DashMap<ThreadId, Histogram<u64>>,
}

impl XMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            total_errors: AtomicU64::new(0),
            total_duration_nanos: AtomicU64::new(0),
            total_data_size: AtomicU64::new(0),
            start_timestamp: Instant::now(),
            per_thread_histograms: DashMap::new(),
        }
    }

    /// Inicia o timer. Zero contenção.
    pub fn Start(&self) {
        CURRENT_START_TIME.with(|cell| {
            *cell.borrow_mut() = Some(Instant::now());
        });
    }

    /// Registra um erro. Operação atômica lock-free.
    pub fn Error(&self) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Finaliza o timer e registra a latência.
    /// Esta função agora tem zero contenção de locks entre threads.
    pub fn End(&self, data_size: u64) {
        if let Some(start_time) = CURRENT_START_TIME.with(|cell| cell.borrow_mut().take()) {
            let duration = start_time.elapsed();
            let duration_nanos = (duration.as_nanos() as u64).max(1);

            // Atualiza contadores globais (lock-free).
            self.total_requests.fetch_add(1, Ordering::Relaxed);
            self.total_duration_nanos.fetch_add(duration_nanos, Ordering::Relaxed);
            self.total_data_size.fetch_add(data_size, Ordering::Relaxed);

            // Obtém o ID da thread atual.
            let thread_id = thread::current().id();

            // Acessa o histograma específico desta thread.
            // Se não existir, o DashMap cria um novo de forma segura.
            // A operação é muito rápida e com granularidade fina.
            let mut hist = self.per_thread_histograms.entry(thread_id).or_insert_with(|| {
                Histogram::new_with_bounds(1, 60_000_000_000, 3).unwrap()
            });

            // Registra a latência no histograma da thread.
            // Nenhuma outra thread está competindo por este histograma.
            hist.record(duration_nanos).unwrap();
        }
    }

    /// Gera o relatório. Esta função agora faz a agregação dos dados de todas as threads.
    /// É mais lenta que antes, mas não impacta o caminho quente.
    pub fn GetMetrics(&self) -> PerformanceMetrics {
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        if total_requests == 0 {
            return PerformanceMetrics::default();
        }

        // Cria um histograma de resumo para agregar os dados.
        let mut summary_histogram = Histogram::new_with_bounds(1, 60_000_000_000, 3).unwrap();

        // Itera sobre todos os histogramas de threads e os adiciona ao resumo.
        for hist_ref in self.per_thread_histograms.iter() {
            summary_histogram.add(hist_ref.value()).unwrap();
        }

        let total_errors = self.total_errors.load(Ordering::Relaxed);
        let total_duration_nanos = self.total_duration_nanos.load(Ordering::Relaxed);
        let total_data = self.total_data_size.load(Ordering::Relaxed);

        let avg_duration_ms = (total_duration_nanos as f64 / total_requests as f64) / 1_000_000.0;
        let elapsed_secs = self.start_timestamp.elapsed().as_secs().max(1);
        let requests_per_sec = total_requests as f64 / elapsed_secs as f64;

        PerformanceMetrics {
            timestamp: self.start_timestamp.elapsed(),
            total_requests: total_requests as f64,
            total_errors: total_errors as i64,
            requests_per_second: requests_per_sec,
            average_latency_ms: avg_duration_ms,
            p50_latency_ms: summary_histogram.value_at_percentile(50.0) as f64 / 1_000_000.0,
            p95_latency_ms: summary_histogram.value_at_percentile(95.0) as f64 / 1_000_000.0,
            p99_latency_ms: summary_histogram.value_at_percentile(99.0) as f64 / 1_000_000.0,
            data_size_kb: (total_data / 1024) as i64,
            data_per_second_kb: (total_data as f64 / elapsed_secs as f64) / 1024.0,
            success_rate: ((total_requests.saturating_sub(total_errors)) as f64 / total_requests as f64) * 100.0,
        }
    }
}

#[derive(Default)]
pub struct PerformanceMetrics {
    pub timestamp: Duration,
    pub total_requests: f64,
    pub total_errors: i64,
    pub requests_per_second: f64,
    pub average_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub data_size_kb: i64,
    pub data_per_second_kb: f64,
    pub success_rate: f64,
}

impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define o locale para o padrão brasileiro (ponto como separador de milhar)
        let pt_br = Locale::pt;

        // Formata os números inteiros com separador de milhar
        let total_requests_fmt = (self.total_requests as i64).to_formatted_string(&pt_br);
        let requests_per_second_fmt = (self.requests_per_second as i64).to_formatted_string(&pt_br);
        let data_size_kb_fmt = self.data_size_kb.to_formatted_string(&pt_br);
        let total_errors_fmt = self.total_errors.to_formatted_string(&pt_br);

        // Formata os números de ponto flutuante, trocando o ponto decimal por vírgula
        let success_rate_fmt = format!("{:.2}", self.success_rate).replace('.', ",");
        let data_per_second_kb_fmt = format!("{:.2}", self.data_per_second_kb).replace('.', ",");
        let average_latency_ms_fmt = format!("{:.3}", self.average_latency_ms).replace('.', ",");
        let p50_latency_ms_fmt = format!("{:.3}", self.p50_latency_ms).replace('.', ",");
        let p95_latency_ms_fmt = format!("{:.3}", self.p95_latency_ms).replace('.', ",");
        let p99_latency_ms_fmt = format!("{:.3}", self.p99_latency_ms).replace('.', ",");

        write!(f,
               "Performance Metrics Report (v3 - Extreme)
=======================================
Tempo de Execução:   {:.2?}
Total de Requisições: {}
Total de Erros:      {}
Taxa de Sucesso:     {}%
---------------------------------------
Requisições/seg:     {}
Dados Totais:        {} KB
Dados/seg:           {} KB/s
---------------------------------------
Latência Média:      {} ms
Latência P50 (Mediana): {} ms
Latência P95:        {} ms
Latência P99:        {} ms",
               self.timestamp,
               total_requests_fmt,
               total_errors_fmt,
               success_rate_fmt,
               requests_per_second_fmt,
               data_size_kb_fmt,
               data_per_second_kb_fmt,
               average_latency_ms_fmt,
               p50_latency_ms_fmt,
               p95_latency_ms_fmt,
               p99_latency_ms_fmt
        );

        writeln!(f, "\nGenerated at: {} UTC", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"))
    }
}
